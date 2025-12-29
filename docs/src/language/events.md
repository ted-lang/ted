# Events

Timed Ted is event-driven. Event handlers execute in response to event sources (signals are one common source).

## The `on` Keyword

The `on` keyword defines event handlers:

```ted
on <event> {
    // code to execute
}
```

`on` introduces a timed context, so `@` and event waits are legal inside the handler.
Handlers run at the current logical time; use `@ +delta` to advance time when needed.

## Event Types

The following adapters are part of the hardware modeling library:

### `rising` - Rising Edge

Triggers when a signal transitions from 0 to 1:

```ted
on rising(clk) {
    count = count + 1;
}
```

### `falling` - Falling Edge

Triggers when a signal transitions from 1 to 0:

```ted
on falling(clk) {
    latch = data;
}
```

### `change` - Any Change

Triggers on any value change:

```ted
on change(data) {
    valid = 1;
    output = process(data);
}
```

Timed events are not limited to hardware signals. The standard library will add event sources like timeouts, intervals, and channels so the same model applies to software and simulations.

## Multiple Events

### Multiple Handlers

You can have multiple handlers for the same signal:

```ted
on rising(clk) {
    // happens first
}

on rising(clk) {
    // happens second
}
```

### Multiple Signals

React to changes in any of several signals:

```ted
on change(a, b, c) {
    result = a + b + c;
}
```

## Event Priority

When multiple events trigger simultaneously:

1. Events are processed in declaration order
2. All handlers for one signal complete before the next
3. Nested events are queued, not immediately executed

## Conditional Events

Use guards to filter events:

```ted
on rising(clk) if enable {
    // only executes if enable is high
    count = count + 1;
}
```

## Comparison with Verilog

| Ted | Verilog |
|-----|---------|
| `on rising(clk)` | `always @(posedge clk)` |
| `on falling(clk)` | `always @(negedge clk)` |
| `on change(sig)` | `always @(sig)` |
| `on change(a, b)` | `always @(a or b)` |

These are convenience adapters; the core semantics are event sources plus timed handlers.

## Best Practices

1. **Keep handlers small** - Each handler should do one thing
2. **Avoid circular triggers** - Don't create infinite event loops
3. **Use rising/falling for clocks** - More efficient than change
4. **Document timing assumptions** - Especially for multi-signal handlers
