# ITP — AQL Language Support for VS Code

Provides syntax highlighting and LSP (Language Server Protocol) support for
**AQL** (ITP Query Language) files (`.aql`).

## Features

- **Syntax highlighting** — keywords, operators, strings, regex, numbers,
  control flow (`let`, `if/then/else/end`, `def/call`, `group by`).
- **Language Server** — completions, hover documentation, diagnostics, and
  formatting via the `itp-lsp` binary.
- **File association** — `.aql` files auto-detected.

## Requirements

- Install `itp` (`cargo install --path src/itp-cli`) to get the `itp-lsp` binary.
- Or set `itp.lspPath` in VS Code settings to the full path.

## Extension Settings

| Setting            | Default     | Description                         |
| ------------------ | ----------- | ----------------------------------- |
| `itp.lspPath`      | `"itp-lsp"` | Path to the itp-lsp binary          |
| `itp.enableLsp`    | `true`      | Enable the AQL Language Server      |
| `itp.trace.server` | `"off"`     | Trace communication with the server |

## Example `.aql` file

```aql
# Find all TODO comments, sorted and de-duplicated
find "TODO" ignore_case | sort | unique | count
```
