# CI/CD

ITP uses GitHub Actions for continuous integration and release automation.

## CI Pipeline (`.github/workflows/ci.yml`)

Runs on every push to `main`/`develop` and on pull requests to `main`.

### Jobs

| Job        | Description                             |
| ---------- | --------------------------------------- |
| `check`    | `cargo check --workspace`               |
| `test`     | Test matrix: Ubuntu, Windows, macOS     |
| `fmt`      | `cargo fmt --check`                     |
| `clippy`   | `cargo clippy -- -D warnings`           |
| `doc`      | `cargo doc --no-deps`                   |
| `wasm`     | Build for `wasm32-unknown-unknown`      |
| `msrv`     | Verify MSRV (Rust 1.75.0)               |
| `security` | `cargo audit` for known vulnerabilities |

## Release Pipeline (`.github/workflows/release.yml`)

Triggered by version tags (`v*.*.*`).

### Build Matrix

| Target                      | OS      |
| --------------------------- | ------- |
| `x86_64-unknown-linux-gnu`  | Ubuntu  |
| `x86_64-unknown-linux-musl` | Ubuntu  |
| `aarch64-unknown-linux-gnu` | Ubuntu  |
| `x86_64-pc-windows-msvc`    | Windows |
| `aarch64-pc-windows-msvc`   | Windows |
| `x86_64-apple-darwin`       | macOS   |
| `aarch64-apple-darwin`      | macOS   |

### Artifacts

Each target produces:
- `itp` — main CLI binary
- `itp-grep` — grep compatibility binary
- `itp-sed` — sed compatibility binary
- `itp-awk` — awk compatibility binary
- `itp-lsp` — LSP server binary

All binaries are compressed and uploaded to GitHub Releases.

### WASM Release

A separate job builds the WASM package with `wasm-pack` and attaches `itp-wasm.tar.gz` to the release.

## Publishing to crates.io

```bash
# Publish in dependency order
cargo publish -p itp-core
cargo publish -p itp-cli
cargo publish -p itp-tui
cargo publish -p itp-gui
cargo publish -p itp-wasm
cargo publish -p itp-lsp
```
