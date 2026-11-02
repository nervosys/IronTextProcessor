//! Line matcher — a non-regex fast path for the cases where `regex::Regex`
//! is overkill.
//!
//! Most grep invocations search for a plain literal. Compiling and running a
//! full regex engine for that case pays for machinery (parsing, NFA/DFA
//! construction, Unicode handling) that a straight substring scan does not
//! need. This module provides a small [`LineMatcher`] trait with three impls:
//!
//! - [`LiteralMatcher`] — a single case-sensitive literal, matched with
//!   `memchr::memmem` (SIMD-accelerated substring search).
//! - [`MultiLiteralMatcher`] — several case-sensitive literals, matched with an
//!   Aho-Corasick automaton (O(n) regardless of pattern count).
//! - [`RegexMatcher`] — the fallback, wrapping `regex::Regex` exactly as
//!   `GrepEngine::new` builds its `compiled` field.
//!
//! [`build_matcher`] returns a literal matcher **only** for the pure-literal
//! fast path (`PatternType::Literal`, case-sensitive, no whole-word). Every
//! other combination — whole-word, case-insensitive, glob, regex, semantic —
//! falls back to [`RegexMatcher`]. Correctness parity with the regex path is
//! non-negotiable: when in doubt, this module defers to [`RegexMatcher`].

use crate::engine::grep::glob_to_regex;
use crate::output::PatternType;
use aho_corasick::AhoCorasick;
use anyhow::{Context, Result};
use regex::{Regex, RegexBuilder};

/// A line matcher: the two operations `grep.rs` needs from a compiled pattern.
///
/// `find_ranges` returns byte ranges (`[start, end)` offsets into `line`),
/// matching what `regex::Regex::find_iter` yields via `mat.start()`/`mat.end()`.
pub trait LineMatcher: Send + Sync {
    /// Does any configured pattern occur in `line`?
    fn is_match(&self, line: &str) -> bool;

    /// Byte ranges of every match on `line`, in left-to-right order,
    /// non-overlapping — the same set `regex::Regex::find_iter` would produce.
    fn find_ranges(&self, line: &str) -> Vec<(usize, usize)>;
}

// ── LiteralMatcher ──────────────────────────────────────────────────────────

/// A single case-sensitive literal, matched with `memchr::memmem`.
///
/// Byte-level substring search is sound here because UTF-8 is
/// self-synchronising: a valid UTF-8 needle can only occur at a char boundary
/// of a valid UTF-8 haystack, so the byte offsets returned always coincide
/// with the char-boundary offsets `regex` reports.
pub struct LiteralMatcher {
    needle: String,
}

impl LiteralMatcher {
    /// Build a literal matcher for `needle` (treated verbatim — any regex
    /// metacharacters it contains are matched literally).
    pub fn new(needle: &str) -> Self {
        Self {
            needle: needle.to_string(),
        }
    }
}

impl LineMatcher for LiteralMatcher {
    fn is_match(&self, line: &str) -> bool {
        // An empty pattern matches everywhere, including the empty line —
        // exactly as a `Regex` built from the empty string does.
        if self.needle.is_empty() {
            return true;
        }
        memchr::memmem::find(line.as_bytes(), self.needle.as_bytes()).is_some()
    }

    fn find_ranges(&self, line: &str) -> Vec<(usize, usize)> {
        if self.needle.is_empty() {
            // A regex built from the empty string yields a zero-width match at
            // every char boundary, including the end of the line. Mirror that
            // so an empty literal stays byte-for-byte parity-correct.
            let mut ranges: Vec<(usize, usize)> =
                line.char_indices().map(|(i, _)| (i, i)).collect();
            ranges.push((line.len(), line.len()));
            return ranges;
        }
        // `memmem::find_iter` yields non-overlapping matches advancing past the
        // end of each one — the same policy `regex::Regex::find_iter` uses.
        let n = self.needle.len();
        memchr::memmem::find_iter(line.as_bytes(), self.needle.as_bytes())
            .map(|start| (start, start + n))
            .collect()
    }
}

// ── MultiLiteralMatcher ─────────────────────────────────────────────────────

/// Several case-sensitive literals, matched with an Aho-Corasick automaton.
pub struct MultiLiteralMatcher {
    automaton: AhoCorasick,
}

impl MultiLiteralMatcher {
    /// Build a multi-literal matcher over `literals`.
    pub fn new(literals: &[&str]) -> Result<Self> {
        let automaton = AhoCorasick::new(literals)
            .context("failed to build Aho-Corasick automaton for literal patterns")?;
        Ok(Self { automaton })
    }
}

impl LineMatcher for MultiLiteralMatcher {
    fn is_match(&self, line: &str) -> bool {
        self.automaton.is_match(line)
    }

    fn find_ranges(&self, line: &str) -> Vec<(usize, usize)> {
        self.automaton
            .find_iter(line)
            .map(|m| (m.start(), m.end()))
            .collect()
    }
}

// ── RegexMatcher ────────────────────────────────────────────────────────────

/// The fallback: a `regex::Regex`, built identically to `GrepEngine::new`.
pub struct RegexMatcher {
    compiled: Regex,
}

impl RegexMatcher {
    /// Wrap an already-compiled regex.
    pub fn new(compiled: Regex) -> Self {
        Self { compiled }
    }
}

impl LineMatcher for RegexMatcher {
    fn is_match(&self, line: &str) -> bool {
        self.compiled.is_match(line)
    }

    fn find_ranges(&self, line: &str) -> Vec<(usize, usize)> {
        self.compiled
            .find_iter(line)
            .map(|m| (m.start(), m.end()))
            .collect()
    }
}

// ── build_matcher ───────────────────────────────────────────────────────────

/// Build the matcher for a pattern configuration.
///
/// Returns a literal/multi-literal matcher **only** for the pure-literal fast
/// path: `PatternType::Literal`, `case_sensitive == true`, `whole_word ==
/// false` (`multiline` is irrelevant to a literal). For every other
/// combination — whole-word, case-insensitive, glob, regex, semantic — it
/// returns a [`RegexMatcher`] built EXACTLY as `GrepEngine::new` builds its
/// `compiled` field (same escaping, same `\b` wrapping, same `RegexBuilder`
/// flags). Correctness parity is non-negotiable; when in doubt we defer here.
pub fn build_matcher(
    pattern_type: PatternType,
    pattern: &str,
    extras: &[String],
    case_sensitive: bool,
    whole_word: bool,
    multiline: bool,
) -> Result<Box<dyn LineMatcher>> {
    // The pure-literal fast path: a plain case-sensitive substring search with
    // no word-boundary constraint. Anything else needs the regex engine.
    if pattern_type == PatternType::Literal && case_sensitive && !whole_word {
        if extras.is_empty() {
            return Ok(Box::new(LiteralMatcher::new(pattern)));
        }
        let mut literals: Vec<&str> = vec![pattern];
        literals.extend(extras.iter().map(|s| s.as_str()));
        return Ok(Box::new(MultiLiteralMatcher::new(&literals)?));
    }

    // Fallback: build the regex exactly as GrepEngine::new does.
    let pattern_str = match pattern_type {
        PatternType::Literal => regex::escape(pattern),
        PatternType::Glob => glob_to_regex(pattern),
        PatternType::Regex | PatternType::Semantic => pattern.to_string(),
    };
    let pattern_str = if whole_word {
        format!(r"\b{}\b", pattern_str)
    } else {
        pattern_str
    };
    let compiled = RegexBuilder::new(&pattern_str)
        .case_insensitive(!case_sensitive)
        .multi_line(multiline)
        .build()
        .with_context(|| format!("Invalid pattern: {}", pattern))?;
    Ok(Box::new(RegexMatcher::new(compiled)))
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// The ground truth for literal parity: a regex built from the escaped
    /// pattern, which is precisely what `GrepEngine::new` compiles today for a
    /// `PatternType::Literal`.
    fn regex_truth_ranges(pattern: &str, line: &str) -> Vec<(usize, usize)> {
        let re = Regex::new(&regex::escape(pattern)).unwrap();
        re.find_iter(line).map(|m| (m.start(), m.end())).collect()
    }

    fn regex_truth_is_match(pattern: &str, line: &str) -> bool {
        let re = Regex::new(&regex::escape(pattern)).unwrap();
        re.is_match(line)
    }

    /// Assert the LiteralMatcher agrees with the escaped-regex ground truth on
    /// BOTH `is_match` and `find_ranges` for `(pattern, line)`.
    fn assert_literal_parity(pattern: &str, line: &str) {
        let lm = LiteralMatcher::new(pattern);
        assert_eq!(
            lm.is_match(line),
            regex_truth_is_match(pattern, line),
            "is_match parity failed for pattern {pattern:?} on line {line:?}"
        );
        assert_eq!(
            lm.find_ranges(line),
            regex_truth_ranges(pattern, line),
            "find_ranges parity failed for pattern {pattern:?} on line {line:?}"
        );
    }

    #[test]
    fn literal_parity_table() {
        // (pattern, line) — the parity suite. Each row is checked for BOTH
        // is_match and find_ranges against a regex built from regex::escape.
        let cases: &[(&str, &str)] = &[
            // literal present / absent (both polarities)
            ("foo", "a foo b"),
            ("foo", "a bar b"),
            // at start / at end
            ("foo", "foo at start"),
            ("foo", "end with foo"),
            // case sensitivity: a case-sensitive literal must match only the
            // same case (guards against the matcher silently going insensitive)
            ("Foo", "foo FOO Foo bar"),
            ("ABC", "abcABCabc"),
            // repeated / overlapping occurrences
            ("aa", "aaaa"),
            ("ab", "ababab"),
            ("ana", "banana"), // overlap: regex is non-overlapping -> one match
            // empty line
            ("foo", ""),
            // empty pattern (matches at every char boundary)
            ("", "abc"),
            ("", ""),
            ("", "café"),
            // multibyte / UTF-8 content
            ("café", "I like café today"),
            ("é", "café résumé"),
            ("日本", "私は日本語を話す日本人"),
            // literals containing regex metacharacters — MUST be literal
            ("a.c", "a.c and abc"),   // '.' must not match 'b'
            ("a.c", "abc only"),      // and must NOT match 'abc'
            ("a*", "a* and aaa"),     // '*' must be literal
            ("[x]", "see [x] here"),  // brackets literal
            ("c++", "c++ rocks"),     // '+' literal
            ("(world)", "hello (world)"),
            ("^start", "^start of line"), // anchors literal
            ("end$", "the end$ here"),
        ];
        for (pattern, line) in cases {
            assert_literal_parity(pattern, line);
        }
    }

    #[test]
    fn literal_metachars_are_not_regex() {
        // The crux of the increment: a literal with metacharacters must behave
        // as a substring, never as a pattern.
        let lm = LiteralMatcher::new("a.c");
        // Matches the literal "a.c"...
        assert!(lm.is_match("xx a.c yy"));
        assert_eq!(lm.find_ranges("xx a.c yy"), vec![(3, 6)]);
        // ...but does NOT match "abc" the way the regex `a.c` would.
        assert!(!lm.is_match("abc"));
        assert!(lm.find_ranges("abc").is_empty());
        // Parity cross-check: escaped-regex agrees.
        assert_eq!(regex_truth_ranges("a.c", "abc"), Vec::<(usize, usize)>::new());
    }

    #[test]
    fn vacuity_guard_non_match() {
        // A case that must NOT match — guards against an "always yes" matcher.
        let lm = LiteralMatcher::new("zzz");
        assert!(!lm.is_match("the quick brown fox"));
        assert!(lm.find_ranges("the quick brown fox").is_empty());
        // And a present case right after, so the guard is not vacuous itself.
        assert!(lm.is_match("buzzz off"));
        assert_eq!(lm.find_ranges("buzzz off"), vec![(2, 5)]);
    }

    #[test]
    fn empty_pattern_matches_every_boundary() {
        // Empty literal == empty regex: zero-width match at every char boundary.
        let lm = LiteralMatcher::new("");
        assert!(lm.is_match(""));
        assert!(lm.is_match("abc"));
        assert_eq!(lm.find_ranges(""), vec![(0, 0)]);
        assert_eq!(lm.find_ranges("abc"), vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
        // café: 'é' is two bytes, so boundaries are 0,1,2,3,5.
        assert_eq!(
            lm.find_ranges("café"),
            regex_truth_ranges("", "café"),
        );
    }

    #[test]
    fn multi_literal_matcher_finds_all() {
        let mlm = MultiLiteralMatcher::new(&["TODO", "FIXME", "HACK"]).unwrap();
        assert!(mlm.is_match("a TODO here"));
        assert!(mlm.is_match("FIXME now"));
        assert!(!mlm.is_match("nothing to see"));
        // Ranges are the automaton's non-overlapping matches, left to right.
        assert_eq!(mlm.find_ranges("TODO and FIXME"), vec![(0, 4), (9, 14)]);
    }

    #[test]
    fn build_matcher_takes_literal_fast_path() {
        // Literal + case-sensitive + no whole-word -> literal fast path, and
        // it must stay parity-correct for a metacharacter literal.
        let m = build_matcher(PatternType::Literal, "a.c", &[], true, false, false).unwrap();
        assert!(m.is_match("a.c"));
        assert!(!m.is_match("abc"));
        assert_eq!(m.find_ranges("a.c here a.c"), vec![(0, 3), (9, 12)]);

        // Multi-literal fast path when extras are present.
        let extras = vec!["bar".to_string()];
        let mm = build_matcher(PatternType::Literal, "foo", &extras, true, false, false).unwrap();
        assert!(mm.is_match("foo"));
        assert!(mm.is_match("bar"));
        assert!(!mm.is_match("baz"));
    }

    #[test]
    fn build_matcher_defers_to_regex_off_fast_path() {
        // Case-insensitive, whole-word, glob and regex all defer to the regex
        // engine, and must behave like GrepEngine's compiled regex.

        // case-insensitive literal
        let ci = build_matcher(PatternType::Literal, "foo", &[], false, false, false).unwrap();
        assert!(ci.is_match("FOO BAR"));

        // whole-word literal: 'cat' matches as a word but not inside 'concat'
        let ww = build_matcher(PatternType::Literal, "cat", &[], true, true, false).unwrap();
        assert!(ww.is_match("a cat sat"));
        assert!(!ww.is_match("concatenate"));

        // a metachar literal under whole-word must still be escaped (literal),
        // not interpreted — so "a.c" does not match "abc".
        let ww_meta = build_matcher(PatternType::Literal, "a.c", &[], true, true, false).unwrap();
        assert!(ww_meta.is_match("an a.c token"));
        assert!(!ww_meta.is_match("an abc token"));

        // genuine regex pattern is honoured as a pattern
        let rx = build_matcher(PatternType::Regex, "a.c", &[], true, false, false).unwrap();
        assert!(rx.is_match("abc"));
        assert_eq!(rx.find_ranges("abc axc"), vec![(0, 3), (4, 7)]);

        // glob is translated the same way grep translates it
        let gl = build_matcher(PatternType::Glob, "*.rs", &[], true, false, false).unwrap();
        assert!(gl.is_match("main.rs"));
        assert!(!gl.is_match("main.py"));
    }
}
