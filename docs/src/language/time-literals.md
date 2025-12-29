# Time and `@`

Ted treats time as an explicit effect in timed contexts. Each timed task carries a current logical time `t`.

## Timed Contexts

`@` and event waiting are only legal in timed contexts, such as:

- `on` handlers
- `loop` blocks inside modules
- (planned) `timed fn` tasks

Core code cannot use `@` and compiles directly with no scheduler overhead.

## Logical Time Model

`@` has one meaning in control flow: advance this task's logical time and yield.

- `@ +delta` advances the task from `t` to `t+delta` and yields
- `x = v` writes at the task's current time `t`
- `x @ -delta` reads a temporal value as it was at `t-delta`
- `x = v @ +delta` is sugar for `@ +delta; x = v`

Here, `delta` is a non-negative time offset (cycles or time units).

Example:

```ted
on change(sig) {
    @ +10ns;
    sig = 1;
}
```

## Temporal Storage

Not every value is time-travelable. Only temporal values keep history:

- Ports and module-level state are temporal today
- Future syntax will make temporal storage explicit (for example, `signal` or `temporal` declarations)

Temporal values keep bounded history so the compiler can allocate compact ring buffers. The history window can be inferred from `@ -delta` uses or declared explicitly.

## Reading the Past

Access previous values of a temporal signal:

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
out delayed: bit,
delayed = input @ -100ns;
```

## Scheduling the Future

Assign values by advancing time:

```ted
// Relative scheduling
sig = 1 @ +1;           // equivalent to: @ +1; sig = 1
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
@ +1;
sig = !prev;

// Delayed feedback
let sample = input @ -1;
@ +10ns;
output = sample;
```

## Rules and Constraints

1. **Timed only** - `@` is only legal in timed contexts
2. **Temporal only** - History reads (`x @ -delta`) require temporal values
3. **Single meaning** - `@ +delta` advances time; `x = v @ +delta` is sugar
4. **Evaluation timing** - `x = v @ +delta` evaluates `v` after the time advance; capture values explicitly if needed
5. **Deterministic** - Ordering is defined; same input produces identical output
6. **Progress** - Timed loops must include time advancement or event waits to avoid zero-time nontermination
7. **Cycle-accurate** - Integer offsets refer to logical cycles
8. **Time-accurate** - Unit-based offsets refer to logical time units
