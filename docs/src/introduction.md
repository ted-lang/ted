# Introduction

> **Note:** Ted is in a very early stage of design. The syntax and semantics described here are subject to change. We welcome feedback and contributions!

**Ted** (Timing-Explicit Description) is a language for hardware simulation that makes time a first-class citizen.

## Philosophy

Ted is built on three core principles:

1. **Time Travel** - The `@` operator lets you read the past and schedule the future naturally
2. **Event-Driven** - React to signal changes without boilerplate
3. **Rust-Like** - Familiar syntax for systems programmers

## A Quick Taste

```ted
mod blink {
    out led: bit,

    loop {
        led = !led @ +500ms;
    }
}
```

Compare this to traditional HDL approaches that require explicit clock declarations, sensitivity lists, and verbose timing constructs.

## The `@` Operator

Ted's signature feature is the time-travel operator `@`:

```ted
// Read the past
let prev = x @ -1;       // x one cycle ago

// Schedule the future
x = 1 @ +10ns;           // x becomes 1 in 10ns
```

This simple syntax replaces Verilog's confusing `#` delays and provides capabilities (like reading past values) that traditional HDLs lack entirely.

## Why Ted?

| Feature | Ted | Verilog |
|---------|-----|---------|
| Future assignment | `x = 1 @ +10ns` | `#10 x = 1` |
| Past reference | `x @ -1` | Not possible |
| Edge detection | `on rising(clk)` | `always @(posedge clk)` |
| Module ports | `in x: bit` | `input x` |

## Next Steps

- [Installation](./getting-started/installation.md) - Get Ted running on your machine
- [Hello World](./getting-started/hello-world.md) - Write your first Ted program
- [Time Literals](./language/time-literals.md) - Deep dive into the `@` operator
