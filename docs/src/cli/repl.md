# REPL

Interactive AQL shell for exploratory text processing.

## Usage

```bash
itp repl [PATHS...]
```

## Features

- Tab completion for AQL keywords
- Command history (persisted across sessions)
- Multi-line input with `\` continuation
- Colored output with syntax highlighting
- Inline help with `help` or `?`

## Commands

| Command | Description                            |
| ------- | -------------------------------------- |
| `help`  | Show available commands and AQL syntax |
| `exit`  | Exit the REPL                          |
| `clear` | Clear the screen                       |
| Any AQL | Execute an AQL query                   |

## Example Session

```
itp> find "TODO"
Found 3 matches in 2 files

itp> find "TODO" | sort | unique
Found 2 unique matches

itp> find "error" ignore_case | count
Count: 42

itp> help
Available AQL stages: find, replace, delete, ...
```
