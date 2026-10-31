# Backward Compatibility

ITP provides drop-in replacement binaries for traditional Unix text tools.

## Binaries

| Binary     | Replaces | Description                             |
| ---------- | -------- | --------------------------------------- |
| `itp-grep` | `grep`   | Pattern matching with structured output |
| `itp-sed`  | `sed`    | Stream editing with dry-run default     |
| `itp-awk`  | `awk`    | Data analysis with typed output         |

## itp-grep

Accepts standard grep flags:

```bash
itp-grep -i "pattern" file.txt          # case-insensitive
itp-grep -r "TODO" src/                 # recursive
itp-grep -c "error" logs/*.log          # count matches
itp-grep -l "TODO" src/                 # files with matches only
itp-grep -n "pattern" file.txt          # line numbers
itp-grep -v "debug" file.txt            # invert match
itp-grep -e "pat1" -e "pat2" file.txt   # multiple patterns
itp-grep -A3 -B3 "error" file.txt       # context lines
```

## itp-sed

Accepts standard sed expression syntax:

```bash
itp-sed 's/old/new/g' file.txt          # substitute
itp-sed -i 's/old/new/g' file.txt       # in-place
itp-sed -e 's/a/b/' -e 's/c/d/' f.txt   # multiple expressions
itp-sed '/pattern/d' file.txt           # delete matching lines
```

## itp-awk

Accepts standard awk flags:

```bash
itp-awk -F',' '{print $1, $3}' data.csv    # field separator
itp-awk '/pattern/ {print $0}' file.txt     # pattern + action
itp-awk 'BEGIN {FS=","} {print $1}' f.csv   # BEGIN block
```

## Key Differences

All backward-compatible binaries add:

- **Structured output** with `--format json/yaml/csv`
- **Dry-run by default** for sed operations (use `--apply` or `-i`)
- **Provenance tracking** in JSON output
- **Better error messages** with suggestions
