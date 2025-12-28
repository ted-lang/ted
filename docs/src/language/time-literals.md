# Time Literals

The `@` operator is Ted's signature feature, enabling intuitive time manipulation.

## The `@` Operator

The `@` operator means "at time". It can be used to:
1. Read values from the past
2. Schedule values for the future

## Reading the Past

Access previous values of any signal:

```ted
// Relative to current time
let prev = x @ -1;        // one cycle ago
let older = x @ -5;       // five cycles ago

// With time units
let past = x @ -10ns;     // 10 nanoseconds ago
let history = x @ -1us;   // 1 microsecond ago
```

### Use Cases

**Edge Detection:**
```ted
let rising_edge = x && !(x @ -1);
let falling_edge = !x && (x @ -1);
```

**Change Detection:**
```ted
let changed = x != (x @ -1);
```

**Delay Line:**
```ted
out delayed: bit,
delayed = input @ -100ns;
```

## Scheduling the Future

Assign values to occur at a future time:

```ted
// Relative scheduling
x = 1 @ +1;           // next cycle
y = value @ +10;      // 10 cycles from now

// With time units
x = 1 @ +10ns;        // in 10 nanoseconds
y = 0 @ +1ms;         // in 1 millisecond
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

## Combining Past and Future

You can use past values to determine future assignments:

```ted
// Invert previous value in the future
x = !(x @ -1) @ +1;

// Delayed feedback
output = input @ -1 @ +10ns;
```

## Rules and Constraints

1. **Past is read-only** - You cannot assign to past values
2. **Future is write-only** - You cannot read scheduled future values
3. **Deterministic** - Multiple assignments to the same future time are resolved by last-write-wins
4. **Cycle-accurate** - Integer offsets refer to simulation cycles
5. **Time-accurate** - Unit-based offsets refer to wall-clock simulation time
