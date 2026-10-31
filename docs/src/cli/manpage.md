# Man Pages

Generate Unix man pages for ITP commands.

## Usage

```bash
# Print man page to stdout
itp manpage

# Generate man pages to a directory
itp manpage --dir /usr/local/share/man/man1
```

## Options

| Flag           | Description                         |
| -------------- | ----------------------------------- |
| `--dir <PATH>` | Output directory for man page files |

When `--dir` is specified, man pages are generated for `itp` and all subcommands. Without `--dir`, the main `itp` man page is printed to stdout for piping to `man -l -`.
