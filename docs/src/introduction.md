# Introduction

> **Note:** Ted is in a very early stage of design. The syntax and semantics described here are subject to change. We welcome feedback and contributions!

**Ted** (Timing-Explicit Description, aka The Teddy Bear Language) is a systems programming language with explicit logical time and deterministic concurrency. Hardware modeling is a first-class library built on the same semantics.

## Philosophy

Ted is built on three core principles:

1. **Time as an effect** - `@` and event waiting are only legal in timed contexts; ordinary code stays direct and fast
2. **Deterministic concurrency** - Scheduling is defined in terms of logical time so runs are reproducible
3. **Systems performance** - Core code compiles to tight machine code; timed code lowers to efficient state machines

## A Quick Taste

```ted
mod blink {
    out led: bit,

    loop {
        led = !led @ +500ms; // schedule the next toggle 500ms later
    }
}
```

This is timed code: the task advances logical time and yields between updates.

## Core vs Timed Ted

- **Core Ted** is ordinary systems code (functions, structs, loops, FFI). It does not involve a scheduler and never uses `@`.
- **Timed Ted** opts into explicit logical time. Timed contexts include `on` handlers and `loop` blocks inside modules today; future `timed fn` will let timed code appear anywhere.

> TODO: Define `timed fn` tasks and where they can appear outside modules.

## Deterministic Concurrency

Scheduling is defined over logical time, so concurrent programs are deterministic by default. This enables reproducible tests and is designed to support replayable debugging and systematic exploration of schedules.

## The `@` Operator

Each timed task has a current logical time `t`:

```ted
on change(sig) {
    // Schedule a future update
    sig = 1 @ +10ns;

    // Read the past (temporal values only)
    let prev = sig @ -1;
}
```

`@` is only legal in timed contexts. Ordinary values have no history; only temporal values support `x @ -delta`.

> TODO: Add a standalone delay statement (`@ +delta;`) for timed code that needs to advance time without assigning.

## Temporal Storage

Not every value is time-travelable. Ports and module-level state are temporal, and future syntax will make temporal storage explicit (for example, a `signal` or `temporal` declaration). Temporal values keep bounded history so the compiler can allocate compact ring buffers and stay fast.

## Hardware Is a Library

Hardware modeling is built on the timed core with libraries and conventions:

- `out led: bit` is sugar for a temporal output type
- `on rising(clk)` is shorthand for subscribing to a rising-edge event stream
- Waveform dumping is a runtime option, not a semantic requirement

## Next Steps

- [Installation](./getting-started/installation.md) - Get Ted running on your machine
- [Hello World](./getting-started/hello-world.md) - Write your first Ted program
- [Time and `@`](./language/time-literals.md) - Deep dive into the timing model
- [Progress](./progress.md) - Track implementation status and planned work
