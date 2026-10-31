# Shell Completions

Generate shell completion scripts for ITP.

## Usage

```bash
itp completions <SHELL>
```

## Supported Shells

| Shell      | Command                      |
| ---------- | ---------------------------- |
| Bash       | `itp completions bash`       |
| Zsh        | `itp completions zsh`        |
| Fish       | `itp completions fish`       |
| PowerShell | `itp completions powershell` |
| Elvish     | `itp completions elvish`     |

## Installation

### Bash

```bash
itp completions bash > ~/.local/share/bash-completion/completions/itp
```

### Zsh

```bash
itp completions zsh > ~/.zfunc/_itp
# Add to .zshrc: fpath+=~/.zfunc; autoload -Uz compinit; compinit
```

### Fish

```bash
itp completions fish > ~/.config/fish/completions/itp.fish
```

### PowerShell

```powershell
itp completions powershell | Out-String | Invoke-Expression
# Or add to $PROFILE for persistence
```
