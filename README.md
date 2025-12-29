# Ted

Ted (Timing-Explicit Description, aka The Teddy Bear Language) is a systems programming language with explicit logical time and deterministic concurrency. Hardware modeling is a first-class library built on the same semantics.

Timing stays an explicit effect: most code is ordinary and compiles to tight CPU code; timed code opts in to `@` and event sources when you need scheduling or simulation.

```ted
mod blink {
    out led: bit,

    loop {
        led = !led @ +500ms;  // equivalent to: @ +500ms; led = !led;
    }
}
```

## Language Model

- **Core Ted** - ordinary functions, data structures, and loops with no scheduler overhead
- **Timed Ted** - `@`, event handlers, and temporal values executed on a deterministic scheduler

## Features

- **Explicit time** - `@ +delta` advances a task's logical time; `x @ -delta` reads history for temporal values
- **Deterministic concurrency** - Same input, same schedule, designed for reproducible runs and replayable debugging
- **Hardware as a library** - Ports, edge events, and waveforms are conventions built on the timed core
- **Rust-like** - Familiar syntax for systems programmers

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

Or read online at [docs.ted-lang.org](https://docs.ted-lang.org).

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
