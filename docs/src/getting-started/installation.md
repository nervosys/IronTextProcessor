# Installation

## From Source

```bash
# Clone the repository
git clone https://github.com/nervosys/IronTextProcessor
cd IronTextProcessor

# Build all binaries
cargo build --release

# Install CLI
cargo install --path src/itp-cli

# Install TUI
cargo install --path src/itp-tui

# Install GUI
cargo install --path src/itp-gui

# Install LSP server
cargo install --path src/itp-lsp
```

Binaries produced: `itp`, `itp-grep`, `itp-sed`, `itp-awk`, `itp-tui`, `itp-gui`, `itp-lsp`.

## Shell Completions

Generate completions for your shell:

```bash
# Bash
itp completions bash > ~/.local/share/bash-completion/completions/itp

# Zsh
itp completions zsh > ~/.zfunc/_itp

# Fish
itp completions fish > ~/.config/fish/completions/itp.fish

# PowerShell
itp completions powershell > $PROFILE.CurrentUserAllHosts
```

## Man Pages

Generate man pages:

```bash
# Single page to stdout
itp manpage

# All pages to a directory
itp manpage --dir /usr/local/share/man/man1
```

## WASM (Browser)

```bash
# Build the WASM package
cd src/itp-wasm
wasm-pack build --target web
```

## Requirements

- Rust 1.75.0+ (MSRV)
- For GUI: platform-specific graphics backend (OpenGL/Vulkan/Metal)
