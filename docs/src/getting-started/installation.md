# Installation

## Prerequisites

Ted requires:
- **Rust** (stable, latest version recommended)
- **Cargo** (comes with Rust)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

## Building from Source

Clone the repository:

```bash
git clone https://github.com/ted-lang/ted.git
cd ted
```

Build the compiler:

```bash
cargo build --release
```

The `ted` binary will be at `target/release/ted`.

## Installation

### Add to PATH

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$PATH:/path/to/ted/target/release"
```

### Or install via Cargo

```bash
cargo install --path crates/ted-cli
```

## Verify Installation

```bash
ted --version
ted --help
```

## Development Setup

For contributing to Ted:

```bash
# Install development tools
rustup component add rustfmt clippy

# Build and test
cargo build --workspace
cargo test --workspace

# Format and lint
cargo fmt --all
cargo clippy --workspace
```

## Editor Support

Ted files use the `.ted` extension. Syntax highlighting is planned for:
- VS Code
- Vim/Neovim
- Emacs

## Next Steps

Now that Ted is installed, continue to [Hello World](./hello-world.md) to write your first program.
