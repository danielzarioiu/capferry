# capferry

`capferry` is an early-stage Rust project, intended as a solid foundation for building a reliable and secure CLI tool.

## Project Status

The repository is currently a minimal Rust scaffold:

- crate initialized with `cargo`
- entrypoint in `src/main.rs`
- no external dependencies yet

## Requirements

- Rust toolchain (recommended via `rustup`)
- Cargo (included with Rust)

Verify installation:

```bash
rustc --version
cargo --version
```

## Quick Start

1. Clone the repository:

```bash
git clone <repo-url>
cd capferry
```

2. Run in development:

```bash
cargo run
```

3. Build release binary:

```bash
cargo build --release
```

4. Run tests:

```bash
cargo test
```

## Project Structure

```text
capferry/
├── Cargo.toml
├── Cargo.lock
└── src/
    └── main.rs
```

## Roadmap (Draft)

- Define the functional scope of `capferry`
- Implement CLI argument parsing (e.g. `clap`)
- Add structured error handling
- Introduce unit and integration tests
- Configure linting and formatting (`clippy`, `rustfmt`)

## Contributing

Contributions are welcome. To contribute:

1. Open an issue to discuss the proposal
2. Create a dedicated branch
3. Submit a pull request with a clear change description

## License

To be defined.
