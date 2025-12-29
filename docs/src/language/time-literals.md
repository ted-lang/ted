# Time and `@`

Ted treats time as an explicit effect in timed contexts. Each timed task carries a current logical time `t`.

## Timed Contexts

`@` and event waiting are only legal in timed contexts, such as:

- `on` handlers
- `loop` blocks inside modules
- (planned) `timed fn` tasks

Core code cannot use `@` and compiles directly with no scheduler overhead.

> TODO: Define `timed fn` syntax and formalize module-level timed tasks in the grammar and parser.

## Logical Time Model

Timed code runs as tasks with a current logical time `t`. Scheduling is expressed with assignment offsets.

- `x = v` writes at the task's current time `t`
- `x = v @ +delta` schedules the write at `t+delta` and yields the task
- `x @ -delta` reads a temporal value as it was at `t-delta`

Here, `delta` is a non-negative time offset (cycles or time units) and can come from a literal or an expression.

Example:

```ted
on change(sig) {
    sig = 1 @ +10ns;
}
```

> TODO: Add a standalone delay statement (`@ +delta;`) for timed code that needs to advance time without assigning.

## Temporal Storage

Not every value is time-travelable. Only temporal values keep history:

- Ports and module-level state are temporal today
- Future syntax will make temporal storage explicit (for example, `signal` or `temporal` declarations)

Temporal values keep bounded history so the compiler can allocate compact ring buffers. The history window can be inferred from `@ -delta` uses or declared explicitly.

> TODO: Specify explicit temporal storage declarations (for example, `signal` or `temporal`) and history bounds.

## Reading the Past

Access previous values of a temporal signal:

Note: The examples in this section assume they appear inside a timed context (for example, an `on` handler or a module-level `loop`).

```ted
// Relative to current time
let prev = sig @ -1;        // one cycle ago
let older = sig @ -5;       // five cycles ago

// With time units
let past = sig @ -10ns;     // 10 nanoseconds ago
let history = sig @ -1us;   // 1 microsecond ago
```

### Use Cases

**Edge Detection:**
```ted
let rising_edge = sig && !(sig @ -1);
let falling_edge = !sig && (sig @ -1);
```

**Change Detection:**
```ted
let changed = sig != (sig @ -1);
```

**Delay Line:**
```ted
on change(input) {
    delayed = input @ -100ns;
}
```

## Scheduling the Future

Schedule values for future logical time:

```ted
// Relative scheduling
sig = 1 @ +1;           // schedule for the next cycle
sig = value @ +10;      // 10 cycles from now

// With time units
sig = 1 @ +10ns;        // in 10 nanoseconds
sig = 0 @ +1ms;         // in 1 millisecond
```

### Use Cases

**Pulse Generation:**
```ted
// Generate a 10ns pulse
pulse = 1;
pulse = 0 @ +10ns;
```

**Delayed Response:**
```ted
on rising(trigger) {
    output = 1 @ +100ns;
}
```

**Periodic Toggle:**
```ted
loop {
    led = !led @ +500ms;
}
```

## Time Units

Ted supports these time units:

| Unit | Meaning |
|------|---------|
| (none) | cycles |
| `ns` | nanoseconds |
| `us` | microseconds |
| `ms` | milliseconds |
| `s` | seconds |

These units are part of logical time; they do not imply wall-clock delays.

## Combining Past and Future

You can use past values to determine future assignments:

```ted
// Capture now, then apply later
let prev = sig @ -1;
sig = !prev @ +1;

// Delayed feedback
let sample = input @ -1;
output = sample @ +10ns;
```

## Rules and Constraints

1. **Timed only** - `@` is only legal in timed contexts
2. **Temporal only** - History reads (`x @ -delta`) require temporal values
3. **Scheduling** - `x = v @ +delta` schedules a write and yields the task
4. **No future reads** - `x @ +delta` is invalid as a read; only assignments can target the future
5. **Evaluation timing** - `x = v @ +delta` evaluates `v` after the time advance; capture values explicitly if needed
6. **Grouping** - Use parentheses for compound expressions (for example, `(a + b) @ +1`)
7. **Deterministic** - Ordering is defined; same input produces identical output
8. **Progress** - Timed loops must include time advancement or event waits to avoid zero-time nontermination
9. **Cycle-accurate** - Integer offsets refer to logical cycles
10. **Time-accurate** - Unit-based offsets refer to logical time units
