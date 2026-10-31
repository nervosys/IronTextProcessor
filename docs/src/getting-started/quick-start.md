# Quick Start

## Search for a Pattern (grep)

```bash
# Search current directory recursively
itp search "TODO" .

# Case-insensitive, with context
itp search -i "error" src/ --context 3

# JSON output for machine consumption
itp search "TODO" . --format json
```

## Transform Text (sed)

```bash
# Preview substitution (dry-run by default)
itp transform 's/old/new/g' file.txt

# Apply changes
itp transform 's/old/new/g' file.txt --apply

# Multiple files
itp transform 's/foo/bar/g' src/ --recursive
```

## Analyze Data (awk)

```bash
# CSV analysis
itp analyze data.csv --separator ','

# Select specific fields
itp analyze data.csv -f 1,3 --separator ','

# With aggregation
itp analyze data.csv --agg count --separator ','
```

## AQL Query

```bash
# Find lines containing a pattern
itp query 'find "TODO"' src/

# Pipeline: find, sort, deduplicate, take top 10
itp query 'find "error" ignore_case | sort | unique | take 10' logs/

# Replace across files
itp query 'replace "old_api" with "new_api"' src/ --apply
```

## Explain Before Executing

```bash
# See what a command would do
itp explain 'find "TODO" | count' src/

# Validate patterns
itp validate "^[a-z]+$"
```
