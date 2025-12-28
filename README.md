# Ted

A timing-explicit language for hardware simulation.

Ted ("The Teddy Bear Language") makes time a first-class citizen with the `@` operator for intuitive time travel semantics.

```ted
mod blink {
    out led: bit,

    loop {
        led = !led @ +500ms;  // toggle every 500ms
    }
}
```

## Features

- **Time Travel** - Read past values with `x @ -1`, schedule future with `x = 1 @ +10ns`
- **Event-Driven** - React to signal changes with `on rising(clk)` / `on change(sig)`
- **Rust-Like** - Familiar syntax for systems programmers
- **Deterministic** - Same input always produces identical output

## Quick Start

```bash
# Build
cargo build --workspace

# Check a source file
cargo run -p ted-cli -- check examples/blink.ted

# Run tests
cargo test --workspace
```

## Documentation

Build and view the docs locally:

```bash
mdbook serve docs/
```

Or read online at [ted-lang.org](https://ted-lang.org) (coming soon).

## Project Structure

```
crates/
├── ted-ir          # Shared IR utilities (spans, IDs)
├── ted-diagnostics # Error reporting
├── ted-lexer       # Tokenization
├── ted-parser      # AST and parsing
└── ted-cli         # Command-line interface
docs/               # Language documentation (mdbook)
examples/           # Example Ted programs
```

## Contributing

See [AGENTS.md](AGENTS.md) for build instructions and coding standards.

## License

Ted is licensed under [GPL-3.0](LICENSE).
