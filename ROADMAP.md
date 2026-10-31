# ITP — Roadmap & Implementation Status

**Project**: Iron Text Processor (ITP)
**Started**: March 2026
**Version**: 3.0.0
**Architecture**: Rust workspace — 6 crates (`itp-core`, `itp-cli`, `itp-tui`, `itp-gui`, `itp-wasm`, `itp-lsp`)

---

## Milestone 1: Core Engines ✅

**Status**: Complete

| Feature                                                               | File                              | Status |
| --------------------------------------------------------------------- | --------------------------------- | ------ |
| Grep engine (14 config options, multi-pattern, context, byte-offsets) | `src/itp-core/src/engine/grep.rs`     | ✅ Done |
| Sed engine (5 command types, address-range, in-place, backup)         | `src/itp-core/src/engine/sed.rs`      | ✅ Done |
| Awk engine (12 expression types, 9 conditions, 8 aggregations)        | `src/itp-core/src/engine/awk.rs`      | ✅ Done |
| Pipeline engine (9 stage types, DSL parser, YAML input)               | `src/itp-core/src/engine/pipeline.rs` | ✅ Done |
| AQL engine (tokenizer, parser, 15 stage types, evaluator)             | `src/itp-core/src/engine/aql.rs`      | ✅ Done |
| File traversal with gitignore, globs, depth limits                    | `src/itp-core/src/traversal.rs`       | ✅ Done |
| Context extraction (function, block, indent, lines, file)             | `src/itp-core/src/context.rs`         | ✅ Done |
| Output formatting (JSON, JSON-pretty, YAML, CSV, Human)               | `src/itp-core/src/output.rs`          | ✅ Done |
| Self-describing ontology (capabilities, commands, types, errors)      | `src/itp-core/src/ontology.rs`        | ✅ Done |

---

## Milestone 2: CLI Commands ✅

**Status**: Complete

| Command     | Aliases                          | File                                | Status |
| ----------- | -------------------------------- | ----------------------------------- | ------ |
| `search`    | `s`, `grep`, `find`              | `src/itp-cli/src/commands/search.rs`    | ✅ Done |
| `transform` | `t`, `sed`, `replace`            | `src/itp-cli/src/commands/transform.rs` | ✅ Done |
| `analyze`   | `a`, `awk`, `fields`             | `src/itp-cli/src/commands/analyze.rs`   | ✅ Done |
| `pipeline`  | `pipe`, `chain`                  | `src/itp-cli/src/commands/pipeline.rs`  | ✅ Done |
| `query`     | `q`, `aql`, `run`                | `src/itp-cli/src/commands/query.rs`     | ✅ Done |
| `ontology`  | `onto`, `capabilities`, `schema` | `src/itp-cli/src/commands/ontology.rs`  | ✅ Done |
| `explain`   | `x`, `preview`                   | `src/itp-cli/src/commands/explain.rs`   | ✅ Done |
| `scope`     | `ls`, `files`                    | `src/itp-cli/src/commands/scope.rs`     | ✅ Done |
| `validate`  | `check`                          | `src/itp-cli/src/commands/validate.rs`  | ✅ Done |
| `context`   | `ctx`                            | `src/itp-cli/src/commands/context.rs`   | ✅ Done |

---

## Milestone 3: AQL — ITP Query Language ✅

**Status**: Complete

| Feature                                                    | Status |
| ---------------------------------------------------------- | ------ |
| Keyword-based syntax design (15 stage types, 11 modifiers) | ✅ Done |
| Tokenizer (strings, regex, keywords, operators)            | ✅ Done |
| Recursive-descent parser                                   | ✅ Done |
| Full evaluator with all modifiers                          | ✅ Done |
| CLI `query` command with `--explain` and `--validate`      | ✅ Done |
| Stdin piping (`cat file \| itp query ...`)                 | ✅ Done |
| Shell pipe chaining (`itp query ... \| itp query ...`)     | ✅ Done |
| SYNTAX.md formal reference                                 | ✅ Done |

---

## Milestone 4: Completeness & Polish ✅

**Status**: Complete

| Task                                                             | File(s)                            | Status |
| ---------------------------------------------------------------- | ---------------------------------- | ------ |
| Add AQL validation to `validate` command                         | `src/itp-cli/src/commands/validate.rs` | ✅ Done |
| Add `query`, `validate`, `context` to ontology                   | `src/itp-core/src/ontology.rs`         | ✅ Done |
| Improve `explain` command (query/scope/validate/context support) | `src/itp-cli/src/commands/explain.rs`  | ✅ Done |
| Fix JSONL output format (true line-delimited)                    | `src/itp-core/src/output.rs`           | ✅ Done |
| TUI: implement Transform tab execution                           | `src/itp-tui/src/app.rs`               | ✅ Done |
| TUI: implement Analyze tab execution                             | `src/itp-tui/src/app.rs`               | ✅ Done |
| TUI: implement Pipeline tab execution                            | `src/itp-tui/src/app.rs`               | ✅ Done |
| GUI: implement Analyze tab                                       | `src/itp-gui/src/app.rs`               | ✅ Done |
| GUI: implement Pipeline tab                                      | `src/itp-gui/src/app.rs`               | ✅ Done |
| Unit tests for awk engine                                        | `src/itp-core/src/engine/awk.rs`       | ✅ Done |
| Unit tests for pipeline engine                                   | `src/itp-core/src/engine/pipeline.rs`  | ✅ Done |
| Unit tests for traversal                                         | `src/itp-core/src/traversal.rs`        | ✅ Done |
| Unit tests for context extraction                                | `src/itp-core/src/context.rs`          | ✅ Done |

---

## Milestone 5: Regulatory Compliance ✅

**Status**: Complete

| Task                                                      | File(s)                              | Status |
| --------------------------------------------------------- | ------------------------------------ | ------ |
| Audit current codebase for security posture               | —                                    | ✅ Done |
| FIPS 180-4 SHA-256 hashing (NIST test vectors verified)   | `src/itp-core/src/compliance.rs`         | ✅ Done |
| Structured audit logging (NIST SP 800-53 AU-3/AU-8)       | `src/itp-core/src/compliance.rs`         | ✅ Done |
| CUI data classification markings (NIST SP 800-171)        | `src/itp-core/src/compliance.rs`         | ✅ Done |
| CMMC 2.0 control mapping (12 controls across 7 domains)   | `src/itp-core/src/compliance.rs`         | ✅ Done |
| Compliance report generation                              | `src/itp-core/src/compliance.rs`         | ✅ Done |
| DataMarking field on ItpEnvelope                          | `src/itp-core/src/output.rs`             | ✅ Done |
| Provenance input_hash activation (with_input_hash)        | `src/itp-core/src/output.rs`             | ✅ Done |
| CLI `compliance` command (report/controls/integrity)      | `src/itp-cli/src/commands/compliance.rs` | ✅ Done |
| Ontology: regulatory_compliance capability + command spec | `src/itp-core/src/ontology.rs`           | ✅ Done |
| COMPLIANCE.md documentation                               | `COMPLIANCE.md`                      | ✅ Done |
| Unit tests for compliance module (11 tests)               | `src/itp-core/src/compliance.rs`         | ✅ Done |

---

## Milestone 6: Backwards Compatibility ✅

**Status**: Complete

| Task                                                    | File(s)                       | Status |
| ------------------------------------------------------- | ----------------------------- | ------ |
| POSIX grep arg parser → `GrepConfig` translation        | `src/itp-core/src/compat.rs`      | ✅ Done |
| POSIX sed arg parser → `SedConfig` translation          | `src/itp-core/src/compat.rs`      | ✅ Done |
| POSIX awk arg/program parser → `AwkConfig` translation  | `src/itp-core/src/compat.rs`      | ✅ Done |
| `itp-grep` binary (POSIX flags + typed output)          | `src/itp-cli/src/bin/itp_grep.rs` | ✅ Done |
| `itp-sed` binary (POSIX flags + typed output)           | `src/itp-cli/src/bin/itp_sed.rs`  | ✅ Done |
| `itp-awk` binary (POSIX flags + typed output)           | `src/itp-cli/src/bin/itp_awk.rs`  | ✅ Done |
| Stdin piping support for all compat binaries            | `src/itp-cli/src/bin/itp_*.rs`    | ✅ Done |
| Auto-format detection (JSON when piped, human when TTY) | `src/itp-cli/src/bin/itp_*.rs`    | ✅ Done |
| Unit tests for compat parsers (22 tests)                | `src/itp-core/src/compat.rs`      | ✅ Done |

---

## Milestone 7: Future Enhancements ✅

**Status**: Complete

| Feature                                                   | Priority | Status                                                         |
| --------------------------------------------------------- | -------- | -------------------------------------------------------------- |
| Streaming execution for large files                       | Medium   | ✅ Done — `search_reader`, `transform_reader`, `process_reader` |
| AQL REPL mode                                             | Medium   | ✅ Done — interactive shell, 10 dot-commands, history           |
| Watch mode for live file monitoring                       | Medium   | ✅ Done — `notify` crate, debounce, `--clear`                   |
| GUI: Apply button for transforms (in-place editing)       | Medium   | ✅ Done — confirmation dialog, backup support                   |
| Improved function detection heuristics                    | Low      | ✅ Done — multi-language, decorators, attributes                |
| Plugin/extension system                                   | Low      | ✅ Done — `ItpPlugin` trait, TOML manifests, registry           |
| Semantic/embedding-based search (`PatternType::Semantic`) | Medium   | ✅ Done — TF-IDF scoring, document ranking                      |
| WASM compilation for browser-based use                    | Low      | ✅ Done — `itp-wasm` crate, wasm-bindgen bindings               |
| LSP integration                                           | Low      | ✅ Done — `itp-lsp` crate, completion/hover/diagnostics         |

---

## Milestone 8: Production Hardening ✅

**Status**: Complete

| Task                                                      | File(s)                                     | Status |
| --------------------------------------------------------- | ------------------------------------------- | ------ |
| Remote telemetry system (CloudWatch-style)                | `src/itp-core/src/telemetry.rs`                 | ✅ Done |
| Semantic search engine (TF-IDF, BM25 scoring)             | `src/itp-core/src/semantic.rs`                  | ✅ Done |
| WASM bindings (grep/sed/awk/aql/semantic/ontology)        | `src/itp-wasm/src/lib.rs`                       | ✅ Done |
| LSP server (completion, hover, diagnostics, formatting)   | `src/itp-lsp/src/server.rs`                     | ✅ Done |
| CI/CD pipeline (8 jobs: lint, test, MSRV, miri, coverage) | `.github/workflows/ci.yml`                  | ✅ Done |
| Release workflows (7 targets, binary artifacts)           | `.github/workflows/release.yml`             | ✅ Done |
| Cross-compilation (5 targets)                             | `Cross.toml`, `.github/workflows/cross.yml` | ✅ Done |
| Criterion benchmarks (grep, sed, awk, aql, semantic)      | `src/itp-core/benches/engines.rs`               | ✅ Done |
| Shell completions (bash, zsh, fish, PowerShell, elvish)   | `src/itp-cli/src/commands/completions.rs`       | ✅ Done |
| Man page generation                                       | `src/itp-cli/src/commands/manpage.rs`           | ✅ Done |
| Integration test suite (21 tests)                         | `src/itp-core/tests/integration.rs`             | ✅ Done |
| Documentation site (mdBook, 27 pages)                     | `docs/`                                     | ✅ Done |
| crates.io publication metadata                            | `Cargo.toml` (all crates)                   | ✅ Done |
| AGPL-3.0-only license                                    | `LICENSE`                                   | ✅ Done |

---

## Milestone 9: v1.0 Release ✅

**Status**: Complete

| Task                                       | Status |
| ------------------------------------------ | ------ |
| Fix all clippy warnings (0 warnings)       | ✅ Done |
| Fix flaky telemetry test (race condition)  | ✅ Done |
| CHANGELOG.md (full release history)        | ✅ Done |
| Update ROADMAP.md for v1.0                 | ✅ Done |
| Update README.md for v1.0                  | ✅ Done |
| Bump version to 1.0.0                      | ✅ Done |
| Final validation (tests, clippy, fmt, doc) | ✅ Done |

---

## Milestone 10: v1.1 — Advanced Features ✅

**Status**: Complete

| Task                                                    | File(s)                          | Status |
| ------------------------------------------------------- | -------------------------------- | ------ |
| AQL v2: `let` variables                                 | `src/itp-core/src/engine/aql.rs`     | ✅ Done |
| AQL v2: `if`/`else`/`end` conditionals                  | `src/itp-core/src/engine/aql.rs`     | ✅ Done |
| AQL v2: `group by` with aggregations                    | `src/itp-core/src/engine/aql.rs`     | ✅ Done |
| AQL v2: `def`/`call` user-defined functions             | `src/itp-core/src/engine/aql.rs`     | ✅ Done |
| AQL v2 unit tests (14 new tests)                        | `src/itp-core/src/engine/aql.rs`     | ✅ Done |
| Parallel file search (rayon)                            | `src/itp-core/src/engine/grep.rs`    | ✅ Done |
| Memory-mapped I/O (memmap2)                             | `src/itp-core/src/engine/grep.rs`    | ✅ Done |
| Benchmark regression CI gate (120% threshold)           | `.github/workflows/ci.yml`       | ✅ Done |
| Configuration system (`~/.atp/config.toml`, `.atprc`)   | `src/itp-core/src/config.rs`         | ✅ Done |
| CLI `config` command (`--show`, `--init`, `--path`)     | `src/itp-cli/src/commands/config.rs` | ✅ Done |
| MCP server mode (4 tools, JSON-RPC 2.0 over stdio)      | `src/itp-cli/src/commands/mcp.rs`    | ✅ Done |
| VS Code extension (AQL grammar, LSP client)             | `editors/vscode/`                | ✅ Done |
| Property-based tests (11 proptest tests)                | `src/itp-core/tests/property.rs`     | ✅ Done |
| Fuzz harness (3 cargo-fuzz targets + corpus)            | `fuzz/`                          | ✅ Done |
| Homebrew formula                                        | `packaging/homebrew/itp.rb`      | ✅ Done |
| Scoop manifest (Windows)                                | `packaging/scoop/itp.json`       | ✅ Done |
| Debian packaging (control, rules, changelog, copyright) | `packaging/debian/`              | ✅ Done |
| RPM spec                                                | `packaging/rpm/itp.spec`         | ✅ Done |

---

## Milestone 11: v1.2 — Async, AI, Code Intel, Remote, Plugin SDK ✅

**Status**: Complete

| Task                                                  | File(s)                           | Status |
| ----------------------------------------------------- | --------------------------------- | ------ |
| Async I/O & streaming (tokio, URL sources, channels)  | `src/itp-core/src/async_io.rs`        | ✅ Done |
| AI/LLM integration (Ollama, OpenAI, NL-to-AQL)        | `src/itp-core/src/ai.rs`              | ✅ Done |
| Code intelligence (14 languages, symbol extraction)   | `src/itp-core/src/code_intel.rs`      | ✅ Done |
| Remote execution (SSH, parallel, host files)          | `src/itp-core/src/remote.rs`          | ✅ Done |
| CLI `ai` command (NL-to-AQL, explain, suggest)        | `src/itp-cli/src/commands/ai.rs`      | ✅ Done |
| CLI `remote` command (SSH distributed execution)      | `src/itp-cli/src/commands/remote.rs`  | ✅ Done |
| CLI `symbols` command (code intelligence queries)     | `src/itp-cli/src/commands/symbols.rs` | ✅ Done |
| CLI `plugin` command (install, scaffold, validate)    | `src/itp-cli/src/commands/plugin.rs`  | ✅ Done |
| TUI overhaul (file browser, AQL tab, Symbols tab)     | `src/itp-tui/src/`                    | ✅ Done |
| Plugin SDK (scaffold, validate, package, marketplace) | `src/itp-core/src/plugin.rs`          | ✅ Done |
| Crates.io publication prep (version deps, dry-run)    | `Cargo.toml` (all crates)         | ✅ Done |

---

## Milestone 12: v1.3 — Index, DAP, WASM, Notebooks, Distributed ✅

**Status**: Complete

| Task                                                       | File(s)                               | Status |
| ---------------------------------------------------------- | ------------------------------------- | ------ |
| Fix flaky telemetry test (USAGE_TEST_MUTEX serialization)  | `src/itp-core/src/telemetry.rs`           | ✅ Done |
| Tree-sitter code intelligence (scope trees, call graphs)   | `src/itp-core/src/code_intel.rs`          | ✅ Done |
| Incremental file index (trigram search, persistence)       | `src/itp-core/src/index.rs`               | ✅ Done |
| DAP integration (AQL debugger, breakpoints, snapshots)     | `src/itp-core/src/dap.rs`                 | ✅ Done |
| WASM plugin runtime (sandbox, capability security)         | `src/itp-core/src/wasm_runtime.rs`        | ✅ Done |
| Notebook / literate mode (markdown + AQL, execute, render) | `src/itp-core/src/notebook.rs`            | ✅ Done |
| Distributed pipelines (scatter/gather, partition, merge)   | `src/itp-core/src/distributed.rs`         | ✅ Done |
| CLI `index` command (build, search, status, update, files) | `src/itp-cli/src/commands/index.rs`       | ✅ Done |
| CLI `debug` command (step, run, info modes)                | `src/itp-cli/src/commands/debug.rs`       | ✅ Done |
| CLI `notebook` command (run, render, info actions)         | `src/itp-cli/src/commands/notebook.rs`    | ✅ Done |
| CLI `distributed` command (scatter/gather execution)       | `src/itp-cli/src/commands/distributed.rs` | ✅ Done |
| 67 new unit tests across 5 new modules                     | —                                     | ✅ Done |

---

## Milestone 13: v1.4 — Streaming, Tracing, WASM Engine, DAP Wire, LSP v2 ✅

**Status**: Complete

| Task                                                         | File(s)                           | Status |
| ------------------------------------------------------------ | --------------------------------- | ------ |
| Aho-Corasick multi-pattern grep                              | `src/itp-core/src/engine/grep.rs`     | ✅ Done |
| OpenTelemetry-style tracing (TracingConfig, TracedOperation) | `src/itp-core/src/telemetry.rs`       | ✅ Done |
| Streaming pipelines (async mpsc, StreamingConfig)            | `src/itp-core/src/engine/pipeline.rs` | ✅ Done |
| LSP v2 (context-aware completion, workspace symbols)         | `src/itp-lsp/src/server.rs`           | ✅ Done |
| LSP notebook diagnostics for `.atp.md` files                 | `src/itp-lsp/src/server.rs`           | ✅ Done |
| TUI v2 (10 tabs: +Index, +Debug, +Notebook, +Distributed)    | `src/itp-tui/src/app.rs`              | ✅ Done |
| Real WASM engine (WasmEngine trait, binary parser)           | `src/itp-core/src/wasm_runtime.rs`    | ✅ Done |
| Project `.atp.toml` (PipelineDefaults, ScopeDefaults)        | `src/itp-core/src/config.rs`          | ✅ Done |
| DAP wire protocol (Content-Length framed JSON encode/decode) | `src/itp-core/src/dap.rs`             | ✅ Done |
| End-to-end integration tests (10 new, 30 total)              | `src/itp-core/tests/integration.rs`   | ✅ Done |
| README.md refresh (badges, diagram, commands, tests)         | `README.md`                       | ✅ Done |
| ~55 new unit tests across all enhanced modules               | —                                 | ✅ Done |

---

## Milestone 14: v1.5 — Cache, Diff, Profile, Schema, Optimizer, Rate Limit, Git, Snapshot ✅

**Status**: Complete

| Task                                                     | File(s)                      | Status |
| -------------------------------------------------------- | ---------------------------- | ------ |
| Incremental compilation cache (SHA-256, LRU, TTL, disk)  | `src/itp-core/src/cache.rs`      | ✅ Done |
| Structured diff engine (JSON diff, text diff, patch)     | `src/itp-core/src/diff.rs`       | ✅ Done |
| Profile-guided optimization hints (4 analysis rules)     | `src/itp-core/src/profile.rs`    | ✅ Done |
| Schema registry (10 JSON Schema 2020-12 definitions)     | `src/itp-core/src/schema.rs`     | ✅ Done |
| Query plan optimizer (pushdown, dead-stage, dedup)       | `src/itp-core/src/optimizer.rs`  | ✅ Done |
| Rate-limited pipelines (token bucket, circuit breaker)   | `src/itp-core/src/rate_limit.rs` | ✅ Done |
| Git-aware search (changed files, blame, line ranges)     | `src/itp-core/src/git_search.rs` | ✅ Done |
| Snapshot testing harness (create, verify, update, norm.) | `src/itp-core/src/snapshot.rs`   | ✅ Done |
| 4 new fuzz targets (pipeline, config, diff, optimizer)   | `fuzz/fuzz_targets/`         | ✅ Done |
| ~90 new unit tests across 8 new modules                  | —                            | ✅ Done |

---

## Milestone 15: v1.6 — Task Queue, Patterns, Checkpoint, Log Sink, Redact, Dep Graph, Template, Workspace, Hooks, Metrics ✅

**Status**: Complete

| Task                                                     | File(s)                      | Status |
| -------------------------------------------------------- | ---------------------------- | ------ |
| Task queue with dependency DAG (Kahn's toposort, retry)  | `src/itp-core/src/task_queue.rs` | ✅ Done |
| Pattern registry (4 built-in sets, 20 patterns)          | `src/itp-core/src/patterns.rs`   | ✅ Done |
| Checkpoint / resume manager (SHA-256, name index, disk)  | `src/itp-core/src/checkpoint.rs` | ✅ Done |
| Structured log sink (5 format parsers, alerts)           | `src/itp-core/src/log_sink.rs`   | ✅ Done |
| Sensitive data redactor (8 rules, 4 mask strategies)     | `src/itp-core/src/redact.rs`     | ✅ Done |
| Dependency graph analyzer (cycles, toposort, DOT export) | `src/itp-core/src/dep_graph.rs`  | ✅ Done |
| Template engine (Mustache-style, partials, iteration)    | `src/itp-core/src/template.rs`   | ✅ Done |
| Workspace aggregator (multi-project discovery, 8 kinds)  | `src/itp-core/src/workspace.rs`  | ✅ Done |
| Event bus / hook system (priority dispatch, skip/abort)  | `src/itp-core/src/hooks.rs`      | ✅ Done |
| Metrics registry (Counter, Gauge, Histogram, Prometheus) | `src/itp-core/src/metrics.rs`    | ✅ Done |
| ~136 new unit tests across 10 new modules                | —                            | ✅ Done |

---

## Milestone 16: v1.7 — Rule Engine, Formatter, Scheduler, Encryption, Changelog, Linter, Data Table, Rewrite, Archive, Report ✅

**Status**: Complete

| Task                                                         | File(s)                       | Status |
| ------------------------------------------------------------ | ----------------------------- | ------ |
| Declarative if-then rule engine (11 conditions, 9 actions)   | `src/itp-core/src/rule_engine.rs` | ✅ Done |
| Table formatter (4 border styles, alignment, overflow)       | `src/itp-core/src/formatter.rs`   | ✅ Done |
| Cron-like scheduler (5-field cron, missed-run detection)     | `src/itp-core/src/scheduler.rs`   | ✅ Done |
| Encryption (PBKDF2-HMAC-SHA256, XOR stream, auth tags)       | `src/itp-core/src/encryption.rs`  | ✅ Done |
| Changelog generator (Conventional Commits, semver bumps)     | `src/itp-core/src/changelog.rs`   | ✅ Done |
| Text/code linter (7 rule types, autofix, file-type filter)   | `src/itp-core/src/linter.rs`      | ✅ Done |
| In-memory data table (filter, sort, group-by, join, pivot)   | `src/itp-core/src/data_table.rs`  | ✅ Done |
| Multi-pass rewrite engine (named rule sets, scope, dry-run)  | `src/itp-core/src/rewrite.rs`     | ✅ Done |
| Archive processor (tar, gzip, search, extract, auto-detect)  | `src/itp-core/src/archive.rs`     | ✅ Done |
| Report composer (Markdown, HTML, JSON, PlainText, templates) | `src/itp-core/src/report.rs`      | ✅ Done |
| ~148 new unit tests across 10 new modules                    | —                             | ✅ Done |

---

## Milestone 17: v1.8 — Converter, Statistics, State Machine, I18n, Batch, Annotation, Highlight, Timeline, Tree, Validator ✅

**Status**: Complete

| Task                                                          | File(s)                         | Status |
| ------------------------------------------------------------- | ------------------------------- | ------ |
| Multi-format converter (JSON, YAML, TOML, CSV, XML, INI)      | `src/itp-core/src/converter.rs`     | ✅ Done |
| Text statistics (readability, entropy, Zipf, n-grams)         | `src/itp-core/src/statistics.rs`    | ✅ Done |
| Programmable state machine (guards, actions, DOT export)      | `src/itp-core/src/state_machine.rs` | ✅ Done |
| Unicode / i18n (normalisation, transliteration, scripts)      | `src/itp-core/src/i18n.rs`          | ✅ Done |
| Transactional batch operations (MemFs, rollback, dry-run)     | `src/itp-core/src/batch.rs`         | ✅ Done |
| Annotation engine (spans, overlap resolution, kappa)          | `src/itp-core/src/annotation.rs`    | ✅ Done |
| Syntax highlighting (5 languages, ANSI/HTML/SVG, themes)      | `src/itp-core/src/highlight.rs`     | ✅ Done |
| Timeline / temporal events (bucketing, anomalies, sparklines) | `src/itp-core/src/timeline.rs`      | ✅ Done |
| Generic tree (DFS/BFS, diff, merge, prune, JSON)              | `src/itp-core/src/tree.rs`          | ✅ Done |
| Multi-format validator (email, URL, semver, UUID, IP, schema) | `src/itp-core/src/validator.rs`     | ✅ Done |
| ~158 new unit tests across 10 new modules                     | —                               | ✅ Done |

---


## Milestone 18: v1.9 — Codec, Fingerprint, Markdown, Sampler, Tokenizer, Spellcheck, Calendar, Compress, Color, Macro Engine ✅

**Status**: Complete

| Task                                                          | File(s)                          | Status |
| ------------------------------------------------------------- | -------------------------------- | ------ |
| Multi-codec encoder/decoder (Base64, Hex, URL, HTML, ROT13)  | `src/itp-core/src/codec.rs`          | ✅ Done |
| Document fingerprinting (SimHash, MinHash, shingling)         | `src/itp-core/src/fingerprint.rs`    | ✅ Done |
| Markdown parser and renderer (AST, HTML, TOC, links)          | `src/itp-core/src/markdown.rs`       | ✅ Done |
| Text sampling (Reservoir, Systematic, Stratified, Weighted)   | `src/itp-core/src/sampler.rs`        | ✅ Done |
| Multi-granularity tokenizer (Word, Sentence, BPE)             | `src/itp-core/src/tokenizer.rs`      | ✅ Done |
| Spell checker (Levenshtein, Soundex, suggestions)             | `src/itp-core/src/spellcheck.rs`     | ✅ Done |
| Date/time utilities (parsing, arithmetic, relative dates)     | `src/itp-core/src/calendar.rs`       | ✅ Done |
| Text compression (RLE, LZ77, Huffman coding)                  | `src/itp-core/src/compress.rs`       | ✅ Done |
| Color manipulation (RGB/HSL, WCAG accessibility, palettes)    | `src/itp-core/src/color.rs`          | ✅ Done |
| Macro engine (11 step types, variables, recorder, recursion)  | `src/itp-core/src/macro_engine.rs`   | ✅ Done |
| ~165 new unit tests across 10 new modules                     | —                                | ✅ Done |

---

## Milestone 19: v2.0 — Fuzzy, Summarizer, Casing, Text Wrap, MIME, Graph, Shell, URL, Table Extract, Emoji ✅

**Status**: Complete

| Task                                                          | File(s)                          | Status |
| ------------------------------------------------------------- | -------------------------------- | ------ |
| Fuzzy string matching (Jaro, Jaro-Winkler, trigrams)          | `src/itp-core/src/fuzzy.rs`          | ✅ Done |
| Text summarization (TF-IDF, TextRank, Lead strategies)        | `src/itp-core/src/summarizer.rs`     | ✅ Done |
| Case conversion and detection (10 styles)                     | `src/itp-core/src/casing.rs`         | ✅ Done |
| Text wrapping (word-wrap, hard-wrap, columns, dedent)         | `src/itp-core/src/text_wrap.rs`      | ✅ Done |
| MIME type detection (magic bytes, extensions, charset)         | `src/itp-core/src/mime.rs`           | ✅ Done |
| Text graph analytics (co-occurrence, PageRank, TextRank)      | `src/itp-core/src/graph.rs`          | ✅ Done |
| Shell command analysis (shebang, portability, escaping)        | `src/itp-core/src/shell.rs`          | ✅ Done |
| URL parsing and manipulation (normalize, resolve, encode)     | `src/itp-core/src/url.rs`            | ✅ Done |
| Table detection and extraction (Markdown, ASCII, fixed-width) | `src/itp-core/src/table_extract.rs`  | ✅ Done |
| Emoji detection and manipulation (find, sentiment, shortcodes)| `src/itp-core/src/emoji.rs`          | ✅ Done |
| ~175 new unit tests across 10 new modules                     | —                                | ✅ Done |

---
## CLI Commands (28 total)

| Command       | Aliases                          | File                                  | Milestone |
| ------------- | -------------------------------- | ------------------------------------- | --------- |
| `search`      | `s`, `grep`, `find`              | `src/itp-cli/src/commands/search.rs`      | 2         |
| `transform`   | `t`, `sed`, `replace`            | `src/itp-cli/src/commands/transform.rs`   | 2         |
| `analyze`     | `a`, `awk`, `fields`             | `src/itp-cli/src/commands/analyze.rs`     | 2         |
| `pipeline`    | `pipe`, `chain`                  | `src/itp-cli/src/commands/pipeline.rs`    | 2         |
| `query`       | `q`, `aql`, `run`                | `src/itp-cli/src/commands/query.rs`       | 2         |
| `ontology`    | `onto`, `capabilities`, `schema` | `src/itp-cli/src/commands/ontology.rs`    | 2         |
| `explain`     | `x`, `preview`                   | `src/itp-cli/src/commands/explain.rs`     | 2         |
| `scope`       | `ls`, `files`                    | `src/itp-cli/src/commands/scope.rs`       | 2         |
| `validate`    | `check`                          | `src/itp-cli/src/commands/validate.rs`    | 2         |
| `context`     | `ctx`                            | `src/itp-cli/src/commands/context.rs`     | 2         |
| `compliance`  | `audit`, `cmmc`                  | `src/itp-cli/src/commands/compliance.rs`  | 5         |
| `repl`        | `shell`, `interactive`           | `src/itp-cli/src/commands/repl.rs`        | 7         |
| `watch`       | `monitor`, `w`                   | `src/itp-cli/src/commands/watch.rs`       | 7         |
| `stream`      | —                                | `src/itp-cli/src/commands/stream.rs`      | 7         |
| `completions` | —                                | `src/itp-cli/src/commands/completions.rs` | 8         |
| `manpage`     | —                                | `src/itp-cli/src/commands/manpage.rs`     | 8         |
| `plugins`     | —                                | `src/itp-cli/src/commands/plugins.rs`     | 7         |
| `config`      | `cfg`                            | `src/itp-cli/src/commands/config.rs`      | 10        |
| `mcp`         | `serve`                          | `src/itp-cli/src/commands/mcp.rs`         | 10        |
| `ai`          | `llm`                            | `src/itp-cli/src/commands/ai.rs`          | 11        |
| `remote`      | `ssh`                            | `src/itp-cli/src/commands/remote.rs`      | 11        |
| `symbols`     | `sym`, `code`                    | `src/itp-cli/src/commands/symbols.rs`     | 11        |
| `plugin`      | `plug`                           | `src/itp-cli/src/commands/plugin.rs`      | 11        |
| `index`       | `idx`                            | `src/itp-cli/src/commands/index.rs`       | 12        |
| `debug`       | `dbg`                            | `src/itp-cli/src/commands/debug.rs`       | 12        |
| `notebook`    | `nb`, `literate`                 | `src/itp-cli/src/commands/notebook.rs`    | 12        |
| `distributed` | `dist`, `scatter`                | `src/itp-cli/src/commands/distributed.rs` | 12        |

---

## Test Coverage

| Module             | Tests                                                 | Status |
| ------------------ | ----------------------------------------------------- | ------ |
| `engine::grep`     | 17 unit tests                                         | ✅      |
| `engine::sed`      | 16 unit tests                                         | ✅      |
| `engine::awk`      | 10 unit tests                                         | ✅      |
| `engine::pipeline` | 15 unit tests                                         | ✅      |
| `engine::aql`      | 42 unit tests + 1 doc-test                            | ✅      |
| `traversal`        | 6 unit tests                                          | ✅      |
| `context`          | 14 unit tests                                         | ✅      |
| `compliance`       | 11 unit tests                                         | ✅      |
| `compat`           | 22 unit tests                                         | ✅      |
| `plugin`           | 8 unit tests                                          | ✅      |
| `telemetry`        | 46 unit tests                                         | ✅      |
| `semantic`         | 13 unit tests                                         | ✅      |
| `output`           | 14 unit tests                                         | ✅      |
| `config`           | 12 unit tests                                         | ✅      |
| Integration tests  | 30 tests                                              | ✅      |
| Property tests     | 11 proptest tests                                     | ✅      |
| itp-lsp            | 25 unit tests                                         | ✅      |
| `async_io`         | 7 unit tests                                          | ✅      |
| `ai`               | 5 unit tests                                          | ✅      |
| `code_intel`       | 17 unit tests                                         | ✅      |
| `remote`           | 12 unit tests                                         | ✅      |
| `index`            | 10 unit tests                                         | ✅      |
| `dap`              | 18 unit tests                                         | ✅      |
| `wasm_runtime`     | 19 unit tests                                         | ✅      |
| `notebook`         | 13 unit tests                                         | ✅      |
| `distributed`      | 13 unit tests                                         | ✅      |
| itp-wasm           | 15 unit tests                                         | ✅      |
| `cache`            | 10 unit tests                                         | ✅      |
| `diff`             | 14 unit tests                                         | ✅      |
| `profile`          | 9 unit tests                                          | ✅      |
| `schema`           | 8 unit tests                                          | ✅      |
| `optimizer`        | 10 unit tests                                         | ✅      |
| `rate_limit`       | 14 unit tests                                         | ✅      |
| `git_search`       | 13 unit tests                                         | ✅      |
| `snapshot`         | 14 unit tests                                         | ✅      |
| `task_queue`       | 13 unit tests                                         | ✅      |
| `patterns`         | 14 unit tests                                         | ✅      |
| `checkpoint`       | 13 unit tests                                         | ✅      |
| `log_sink`         | 14 unit tests                                         | ✅      |
| `redact`           | 14 unit tests                                         | ✅      |
| `dep_graph`        | 15 unit tests                                         | ✅      |
| `template`         | 14 unit tests                                         | ✅      |
| `workspace`        | 12 unit tests                                         | ✅      |
| `hooks`            | 13 unit tests                                         | ✅      |
| `metrics`          | 14 unit tests                                         | ✅      |
| `rule_engine`      | 18 unit tests                                         | ✅      |
| `formatter`        | 16 unit tests                                         | ✅      |
| `scheduler`        | 16 unit tests                                         | ✅      |
| `encryption`       | 14 unit tests                                         | ✅      |
| `changelog`        | 16 unit tests                                         | ✅      |
| `linter`           | 14 unit tests                                         | ✅      |
| `data_table`       | 15 unit tests                                         | ✅      |
| `rewrite`          | 14 unit tests                                         | ✅      |
| `archive`          | 13 unit tests                                         | ✅      |
| `report`           | 12 unit tests                                         | ✅      |
| `converter`        | 17 unit tests                                         | ✅      |
| `statistics`       | 15 unit tests                                         | ✅      |
| `state_machine`    | 16 unit tests                                         | ✅      |
| `i18n`             | 16 unit tests                                         | ✅      |
| `batch`            | 15 unit tests                                         | ✅      |
| `annotation`       | 16 unit tests                                         | ✅      |
| `highlight`        | 16 unit tests                                         | ✅      |
| `timeline`         | 15 unit tests                                         | ✅      |
| `tree`             | 16 unit tests                                         | ✅      |
| `validator`        | 16 unit tests                                         | ✅      |
| Fuzz targets       | 7 (aql, grep, sed, pipeline, config, diff, optimizer) | ✅      |
| `codec`            | 17 unit tests                                         | ✅      |
| `fingerprint`      | 17 unit tests                                         | ✅      |
| `markdown`         | 17 unit tests                                         | ✅      |
| `sampler`          | 15 unit tests                                         | ✅      |
| `tokenizer`        | 16 unit tests                                         | ✅      |
| `spellcheck`       | 16 unit tests                                         | ✅      |
| `calendar`         | 18 unit tests                                         | ✅      |
| `compress`         | 15 unit tests                                         | ✅      |
| `color`            | 18 unit tests                                         | ✅      |
| `macro_engine`     | 16 unit tests                                         | ✅      |
| `fuzzy`            | 19 unit tests                                         | ✅      |
| `summarizer`       | 16 unit tests                                         | ✅      |
| `casing`           | 23 unit tests                                         | ✅      |
| `text_wrap`        | 16 unit tests                                         | ✅      |
| `mime`             | 17 unit tests                                         | ✅      |
| `graph`            | 15 unit tests                                         | ✅      |
| `shell`            | 17 unit tests                                         | ✅      |
| `url`              | 18 unit tests                                         | ✅      |
| `table_extract`    | 17 unit tests                                         | ✅      |
| `emoji`            | 17 unit tests                                         | ✅      |
| **Total**          | **~1235 tests + 1 doc-test**                           | ✅      |

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                       ITP Ecosystem                          │
├─────────┬─────────┬─────────┬──────────┬─────────┬──────────┤
│  itp    │ itp-tui │ itp-gui │ itp-wasm │ itp-lsp │  Agent   │
│  (CLI)  │  (TUI)  │  (GUI)  │  (WASM)  │  (LSP)  │via JSON  │
├─────────┴─────────┴─────────┴──────────┴─────────┴──────────┤
│            POSIX Compatibility Binaries                      │
│  ┌───────────┐ ┌───────────┐ ┌───────────┐                  │
│  │ itp-grep  │ │ itp-sed   │ │ itp-awk   │                  │
│  └─────┬─────┘ └─────┬─────┘ └─────┬─────┘                  │
│        └──────────────┼──────────────┘                        │
│                  compat.rs                                    │
├──────────────────────────────────────────────────────────────┤
│                       itp-core                                │
│  ┌───────────────────────────────────────────────────────┐   │
│  │              AQL Engine (Unified)                      │   │
│  │   Tokenizer → Parser → AST → Evaluator                │   │
│  └───────────────────────────────────────────────────────┘   │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │  Grep    │ │  Sed     │ │  Awk     │ │ Semantic │        │
│  │  Engine  │ │  Engine  │ │  Engine  │ │  Search  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │ Pipeline │ │ Ontology │ │ Context  │ │Telemetry │        │
│  │  Engine  │ │  System  │ │ Extract  │ │  System  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │Traversal │ │  Output  │ │Compliance│ │  Plugin  │        │
│  │  Walker  │ │ Formatter│ │  Module  │ │  System  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │ Async IO │ │    AI    │ │  Code    │ │  Remote  │        │
│  │ Streaming│ │  Engine  │ │  Intel   │ │  Executor│        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │  Index   │ │   DAP    │ │  WASM    │ │ Notebook │        │
│  │  Engine  │ │ Debugger │ │ Runtime  │ │ Literate │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │Distribute│ │  Cache   │ │   Diff   │ │ Profile  │        │
│  │ Pipeline │ │  Engine  │ │  Engine  │ │  Hints   │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │  Schema  │ │Optimizer │ │RateLimit │ │   Git    │        │
│  │ Registry │ │  (AQL)   │ │ Executor │ │  Search  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │ Snapshot │ │TaskQueue │ │ Patterns │ │Checkpoint│        │
│  │  Harness │ │  (DAG)   │ │ Registry │ │  Resume  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │ Log Sink │ │  Redact  │ │ DepGraph │ │ Template │        │
│  │  Parser  │ │  Engine  │ │ Analyzer │ │  Engine  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐                     │
│  │Workspace │ │  Hooks   │ │ Metrics  │                     │
│  │Aggregator│ │EventBus  │ │ Registry │                     │
│  └──────────┘ └──────────┘ └──────────┘                     │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │  Rule    │ │Formatter │ │Scheduler │ │Encryption│        │
│  │  Engine  │ │  Tables  │ │  Cron    │ │  Module  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │Changelog │ │  Linter  │ │DataTable │ │ Rewrite  │        │
│  │Generator │ │  Engine  │ │ Columnar │ │  Engine  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐                                   │
│  │ Archive  │ │  Report  │                                   │
│  │Processor │ │ Composer │                                   │
│  └──────────┘ └──────────┘                                   │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │Converter │ │Statistic │ │  State   │ │   I18n   │        │
│  │ Formats  │ │  Engine  │ │ Machine  │ │ Unicode  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │  Batch   │ │Annotator │ │Highlight │ │ Timeline │        │
│  │  Ops     │ │  Engine  │ │  Syntax  │ │  Events  │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  ┌──────────┐ ┌──────────┐                                   │
│  │   Tree   │ │Validator │                                   │
│  │Structure │ │  Multi   │                                   │
│  └──────────┘ └──────────┘                                   │
└──────────────────────────────────────────────────────────────┘
```
