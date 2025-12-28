# AGENTS.md — Ted (Timing-Explicit Description) Build & Contributor Guide

This repository contains **Ted** (“**The Teddy Bear Language**”), a **Rust-based compiler toolchain** for a **timing-explicit** language aimed at **hardware simulation** (cycle/event semantics, waveforms, determinism) while also being **high-performance on CPU** (fast simulation kernels, optional JIT/AOT paths, SIMD-friendly data layouts).

This file is written for:
- automated coding agents (LLMs, CI bots) making changes safely and predictably
- human contributors who want a clear build + quality contract

---

## Goals and non-negotiables

### Primary goals
- **Deterministic simulation:** Same input must produce identical outputs across runs and platforms (within documented limits).
- **Timing is explicit:** No hidden scheduling. Delays, clocks, and event ordering are explicit in IR and in the simulator.
- **CPU performance:** Avoid “cute” implementations. Favor cache-friendly layouts, fewer allocations, and predictable hot loops.
- **Excellent diagnostics:** Compiler errors must explain timing and causality (who scheduled what, when).

### Non-goals (at least initially)
- Full HDL synthesis parity with Verilog/VHDL.
- Implicit concurrency semantics that hide scheduling rules.

---

## Repository layout (recommended / expected)

Ted is a Cargo workspace. Keep crates small and single-purpose.

```
.
├─ crates/
│  ├─ ted-lexer/        # tokenization
│  ├─ ted-parser/       # AST + parsing
│  ├─ ted-hir/          # high-level IR (typed, name-resolved)
│  ├─ ted-mir/          # timing-explicit mid-level IR (scheduled/events)
│  ├─ ted-ir/           # shared IR utilities (IDs, arenas, spans)
│  ├─ ted-diagnostics/  # error reporting (spans, labels, rendering)
│  ├─ ted-sim/          # simulator runtime + kernels (cycle/event)
│  ├─ ted-codegen/      # backends (C/LLVM/Cranelift/Verilog, optional)
│  ├─ ted-cli/          # `ted` command line interface
│  └─ ted-tests/        # integration tests + golden files (optional)
├─ examples/            # small programs; must stay up to date
├─ docs/                # documentation (using mdbook)
├─ tests/               # integration tests (if not in a crate)
├─ benches/             # criterion benches (or inside crates)
├─ AGENTS.md
└─ Cargo.toml
```

If the repo differs, do not “fix” layout wholesale. Instead, adjust this file and keep changes incremental.

---

## Toolchain and prerequisites

### Required
- **Rust stable** (latest stable recommended).
- `rustfmt` and `clippy` components.

Install:
```bash
rustup update stable
rustup component add rustfmt clippy
````

### Strongly recommended (for performance work)

* `llvm-tools-preview` (profiling / symbolization workflows)
* Linux perf tools or macOS Instruments as applicable

```bash
rustup component add llvm-tools-preview
```

### Optional dependencies (only if enabled by features)

* **Cranelift JIT** (pure Rust dependency; no external install usually required)
* **LLVM-based backend** (if present) may require an LLVM installation depending on crate choices

Do not introduce new native dependencies unless there is a clear performance or compatibility benefit.

---

## Quickstart: build, test, run

### Build everything

```bash
cargo build --workspace
```

### Run the CLI

```bash
cargo run -p ted-cli -- --help
```

### Run a simple compile + simulate (example)

(Adjust paths/options to match the actual CLI.)

```bash
cargo run -p ted-cli -- compile examples/blink.ted -o /tmp/blink.tedbc
cargo run -p ted-cli -- sim /tmp/blink.tedbc --cycles 100 --vcd /tmp/blink.vcd
```

### Run tests

```bash
cargo test --workspace
```

### Format and lint (must pass in CI)

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

---

## Feature flags (conventions)

Prefer a small set of clearly named features.

Suggested features:

* `sim` — simulator runtime (usually default)
* `jit` — JIT backend (e.g., Cranelift)
* `verilog` — Verilog emission backend (if implemented)
* `vcd` — waveform dumping
* `bench` — enable benchmark-only code paths
* `trace` — debug tracing (must be off by default)

Build with all features (CI sanity):

```bash
cargo build --workspace --all-features
cargo test  --workspace --all-features
```

Rules:

* Features must be additive and documented in crate READMEs.
* Default features should keep builds fast and dependency footprint reasonable.

---

## “Definition of Done” for any change

An agent should not stop at “it compiles”. Every PR/change must satisfy:

1. `cargo fmt --all` produces no diff
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
3. `cargo test --workspace` passes
4. If semantics changed: add/update at least one **golden test** or **integration test**
5. If performance-sensitive: add a **benchmark** or include a measurable before/after note

If you can’t run something locally (e.g., platform tooling), document that limitation in the PR description.

---

## Coding standards

### Rust style

* Prefer **safe Rust**; `unsafe` is allowed only for hot paths with measured wins.
* Avoid panics in library crates; return diagnostics or structured errors.
* Use `#[must_use]` on fallible builders / result-like APIs where helpful.
* Keep public APIs minimal and documented.

### Determinism policy (simulation)

* No iteration over `HashMap` / `HashSet` where order affects results.

  * Use `BTreeMap`, `IndexMap` (with stable iteration), or sort keys explicitly.
* Use explicit integer widths for timing and bit-accurate values (`u64`, `i64`, or custom bitvectors).
* Floating point is allowed only when explicitly part of the language and must be documented as potentially non-bitwise-deterministic across platforms.

### Error messages and spans

* Every user-facing error must:

  * include a span
  * name the phase (parse/name/type/schedule/sim)
  * suggest a fix when possible

### Logging

Use `tracing` (preferred) or `log` consistently across the workspace.

* Default: quiet
* Debug via environment variables:

  ```bash
  RUST_LOG=ted=debug cargo run -p ted-cli -- ...
  ```

---

## Architecture notes (how to think about Ted)

### Frontend pipeline

Typical flow (names may vary by crate):

1. **Lex + parse** → AST (lossless tokens/spans)
2. **Name resolution** → HIR (IDs instead of strings)
3. **Type checking** → typed HIR
4. **Lowering** → MIR / timing IR (explicit events, delays, clocks)
5. **Backend** → simulator / JIT / codegen / Verilog emission

Keep transformations explicit and testable. Each phase should have:

* a stable input/output representation
* unit tests for edge cases
* snapshot tests for diagnostics (if used)

### Timing-explicit semantics (core concept)

Ted programs must lower into an IR where:

* time advancement is explicit (`delay`, `wait`, `at`, `on clock`)
* event ordering is explicit (priority, tie-breaking rules documented)
* concurrent processes are modeled as **scheduled continuations** or **state machines**

Avoid “magic concurrency”. Prefer:

* explicit event queues
* explicit clock domains
* explicit delta-cycles (if supported)

---

## Simulator performance principles

The simulator (`crates/ted-sim`) is performance-critical.

### Hot loop guidance

* Avoid per-event allocations. Use arenas, slabs, and reuse buffers.
* Prefer dense IDs (newtype indices) and `Vec`-backed tables:

  * `SignalId(u32)`, `ProcessId(u32)`, etc.
* Use SoA (struct-of-arrays) where it improves cache use.
* Avoid virtual dispatch in the inner loop.
* Keep “waveform dumping” off the hot path:

  * gated behind a flag
  * use buffered IO
  * allow decimation / signal selection

### Simulation modes

If multiple are supported, keep them separate and test both:

* **Cycle-based**: best for synchronous designs; fixed tick loop
* **Event-driven**: best for sparse activity; priority queue / timing wheel

### Deterministic scheduling

Document and enforce tie-breaking:

* same timestamp events
* delta-cycle ordering (if applicable)
* stable iteration over ready queues

If behavior is unspecified, tests must not depend on it.

---

## Testing strategy

### Unit tests

* Parsing: tricky grammar, recovery, spans
* Name/type: shadowing, generics (if any), width inference (if any)
* MIR lowering: timing constructs, scheduling rules
* Simulator: event ordering, race edge cases, reset semantics

Run:

```bash
cargo test -p ted-parser
cargo test -p ted-sim
```

### Integration / golden tests

Prefer “golden” tests for:

* diagnostics text (stable formatting)
* MIR dumps (for tricky scheduling)
* waveform summaries (not full VCD unless small)

Common patterns:

* `tests/ui/*.ted` + `tests/ui/*.stderr`
* `tests/golden/*.ted` + `*.mir` / `*.out`

If using snapshot testing (e.g., `insta`), document update flow:

```bash
cargo insta accept
```

### Long-running tests

Mark as ignored:

```bash
cargo test -- --ignored
```

---

## Benchmarks and profiling

### Criterion benches

Add benches for hot kernels:

* event queue operations
* signal update propagation
* process scheduling overhead
* VCD emission overhead (separately)

Run:

```bash
cargo bench -p ted-sim
```

### Profiling (suggested)

* Linux:

  * `perf record` / `perf report`
* macOS:

  * Instruments “Time Profiler”

Do not merge an “optimization” without either:

* benchmark evidence, or
* a clear algorithmic improvement with expected wins explained

---

## Adding a new language feature (agent checklist)

When implementing a new Ted feature:

1. Update grammar + parser tests
2. Extend HIR and type rules (if applicable)
3. Lower into timing-explicit MIR
4. Ensure simulator semantics are deterministic and documented
5. Add at least:

   * 1 positive test
   * 1 negative test (diagnostic)
6. Update an example program if the feature is user-facing

If a feature changes timing behavior, include a MIR dump or scheduling test that proves ordering.

---

## Backends (codegen) guidance

Backends must share the same semantics as the interpreter/simulator:

* If there is a JIT backend, it must match event ordering exactly.
* If there is Verilog emission, document what subset is supported and how timing maps.

Never “fix” a backend by changing semantics silently. Update tests and document the change.

## Docs guideline
the docs/ directory should contain user-facing documentation built with mdbook. Keep it updated with new features and changes. It will be deployed at https://docs.ted-lang.org.

---

## Security / safety notes

* Treat input programs as untrusted.
* Avoid uncontrolled recursion in parser/type checker where possible.
* Any file emission (VCD, artifacts) must:

  * not overwrite unexpectedly
  * create directories intentionally
  * validate paths when used in automated contexts

---

## CI contract (what the project should enforce)

Recommended CI steps:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
```

If CI is failing due to toolchain drift, update the pinned toolchain (if used) and document the reason.

---

## Agent operating rules (how to make changes)

When acting as an automated agent in this repo:

* Make **small, reviewable commits**.
* Avoid sweeping refactors unless explicitly requested.
* Prefer adding tests over adding comments explaining why it “should work”.
* If you must introduce `unsafe`, include:

  * a comment explaining invariants
  * a focused benchmark showing benefit
  * a safe fallback if feasible

Always leave the repo in a state where `cargo test --workspace` can run successfully.

---

## Useful commands (copy/paste)

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint + fmt (CI-equivalent)
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run CLI
cargo run -p ted-cli -- --help

# Debug logging
RUST_LOG=ted=debug cargo run -p ted-cli -- compile examples/blink.ted

# All-features sanity
cargo test --workspace --all-features
```
