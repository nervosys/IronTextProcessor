# Transform (sed)

Transform text using substitution, deletion, insertion, and transliteration.

## Usage

```bash
itp transform <EXPRESSION> [PATHS...] [OPTIONS]
```

## Expression Syntax

| Expression     | Description                   |
| -------------- | ----------------------------- |
| `s/old/new/`   | Substitute first occurrence   |
| `s/old/new/g`  | Substitute all occurrences    |
| `s/old/new/gi` | Global, case-insensitive      |
| `d/pattern/`   | Delete lines matching pattern |
| `y/abc/xyz/`   | Transliterate characters      |

## Options

| Flag                    | Description                           |
| ----------------------- | ------------------------------------- |
| `--apply`               | Apply changes (default is dry-run)    |
| `-e, --expression`      | Additional expressions                |
| `--address <START,END>` | Limit to lines matching address range |
| `--line-range <S-E>`    | Limit to line number range            |
| `--recursive`           | Process directories recursively       |
| `--backup`              | Create backup files before modifying  |

## Examples

```bash
# Preview a substitution
itp transform 's/foo/bar/g' src/

# Apply changes
itp transform 's/old_api/new_api/g' src/ --apply --recursive

# Delete blank lines
itp transform 'd/^$/' file.txt --apply

# Multiple expressions
itp transform 's/a/b/g' -e 's/c/d/g' file.txt

# Address range (between START and END patterns)
itp transform 's/x/y/g' file.txt --address 'BEGIN,END'
```

## Output Schema

```json
{
  "pattern": "foo",
  "replacement": "bar",
  "total_changes": 12,
  "files_modified": 3,
  "applied": false,
  "changes": [
    {
      "file": "src/main.rs",
      "change_count": 4,
      "changes": [
        {
          "line_number": 10,
          "original": "let foo = 42;",
          "replacement": "let bar = 42;",
          "change_type": "Substitution"
        }
      ]
    }
  ]
}
```
