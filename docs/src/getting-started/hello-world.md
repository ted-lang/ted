# Hello World

Let's write your first Ted program.

## The Simplest Program

Create a file called `hello.ted`:

```ted
mod hello {
    out message: u8,

    message = 42;
}
```

This module outputs the constant value 42.

## Check the Program

```bash
ted check hello.ted
```

If there are no errors, you'll see:

```
No errors found.
```

## A Blinking LED

A more interesting example - an LED that toggles every 500ms:

```ted
mod blink {
    out led: bit,

    loop {
        led = !led @ +500ms;
    }
}
```

This demonstrates:
- `out led: bit` - an output signal
- `loop { ... }` - continuous execution
- `!led` - toggle the current value
- `@ +500ms` - schedule the change 500ms in the future

## A Counter

A classic 8-bit counter:

```ted
mod counter {
    in  clk: bit,
    in  reset: bit,
    out count: u8,

    on rising(clk) {
        if reset {
            count = 0;
        } else {
            count = count + 1;
        }
    }
}
```

This demonstrates:
- Multiple ports (`in` and `out`)
- `on rising(clk)` - react to clock edges
- Conditional logic with `if`/`else`

## Using Past Values

Detect a rising edge manually:

```ted
mod edge_detect {
    in  signal: bit,
    out rising: bit,

    on change(signal) {
        rising = signal && !(signal @ -1);
    }
}
```

The `@ -1` reads the signal's value from one cycle ago.

## Next Steps

- [Syntax Overview](../language/syntax.md) - Learn the full syntax
- [Time Literals](../language/time-literals.md) - Master the `@` operator
- [Examples](../examples/README.md) - More complete examples
