# CLI Reference

ITP provides 15 subcommands accessible via the `itp` binary:

## Core Operations

| Command     | Description                              | Alias      |
| ----------- | ---------------------------------------- | ---------- |
| `search`    | Search files for patterns (grep)         | `itp-grep` |
| `transform` | Transform text with substitutions (sed)  | `itp-sed`  |
| `analyze`   | Analyze structured data (awk)            | `itp-awk`  |
| `query`     | Execute AQL queries                      |            |
| `pipeline`  | Run multi-stage pipelines from YAML/JSON |            |

## Inspection

| Command      | Description                        |
| ------------ | ---------------------------------- |
| `ontology`   | Machine-readable capability schema |
| `explain`    | Preview what a command would do    |
| `validate`   | Check pattern validity             |
| `compliance` | FIPS/CMMC compliance report        |

## Interactive

| Command  | Description                      |
| -------- | -------------------------------- |
| `repl`   | Interactive AQL shell            |
| `watch`  | Watch files and re-run on change |
| `stream` | Process stdin in streaming mode  |

## Tooling

| Command       | Description                |
| ------------- | -------------------------- |
| `completions` | Generate shell completions |
| `manpage`     | Generate man pages         |
| `plugins`     | List installed plugins     |

## Global Flags

```
--format <FORMAT>    Output format: json, json-pretty, jsonl, yaml, csv, human
--no-color           Disable colored output
--verbose            Verbose output
--quiet              Suppress non-essential output
```

See individual command pages for details.
