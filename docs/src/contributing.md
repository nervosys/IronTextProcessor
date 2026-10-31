# Contributing

## Development Setup

```bash
git clone https://github.com/nervosys/IronTextProcessor
cd IronTextProcessor
cargo build
cargo test --workspace
```

## Code Style

- `cargo fmt` — Format all code
- `cargo clippy` — Lint all code
- MSRV: Rust 1.75.0

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p itp-core
cargo test -p itp-lsp

# Run integration tests
cargo test -p itp-core --test integration

# Run benchmarks
cargo bench -p itp-core
```

## Project Structure

| Crate      | Purpose                                |
| ---------- | -------------------------------------- |
| `itp-core` | Core library — engines, AQL, telemetry |
| `itp-cli`  | CLI binary and subcommand handlers     |
| `itp-tui`  | Terminal UI                            |
| `itp-gui`  | Desktop GUI                            |
| `itp-wasm` | WebAssembly bindings                   |
| `itp-lsp`  | LSP server for AQL files               |

## Adding a New Engine Feature

1. Implement the feature in `src/itp-core/src/engine/`
2. Add output types to `src/itp-core/src/output.rs` if needed
3. Expose via CLI in `src/itp-cli/src/commands/`
4. Add WASM binding in `src/itp-wasm/src/lib.rs` if applicable
5. Write unit tests in the module
6. Add integration tests in `src/itp-core/tests/integration.rs`
7. Add benchmarks in `src/itp-core/benches/engines.rs`

## Adding an AQL Stage

1. Add the variant to `AqlStage` enum in `src/itp-core/src/engine/aql.rs`
2. Implement parsing in the `parse_stage` function
3. Implement execution in the `execute_stage` function
4. Add `describe()` output for `--explain`
5. Add LSP keyword to `src/itp-lsp/src/server.rs`
6. Add tests

## License

AGPL-3.0-only — see [LICENSE](https://github.com/nervosys/IronTextProcessor/blob/master/LICENSE). Commercial licensing available — contact [opensource@nervosys.ai](mailto:opensource@nervosys.ai).
