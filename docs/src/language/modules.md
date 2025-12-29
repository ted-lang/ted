# Modules

Modules are the current top-level unit in Ted. Hardware modeling uses ports and event handlers, but core code can live inside modules without any timing.

## Module Declaration

```ted
mod module_name {
    // ports
    // internal signals
    // logic
}
```

## Ports

Ports define the module's interface for hardware modeling:

```ted
mod example {
    in  clock: bit,           // input port
    out data: u8,             // output port
    inout bus: u32,           // bidirectional port
}
```

### Port Directions

| Direction | Description |
|-----------|-------------|
| `in` | Input - read only inside module |
| `out` | Output - write only inside module |
| `inout` | Bidirectional - read and write |

### Port Types

Ports must have explicit types:

```ted
in single_bit: bit,
in byte_value: u8,
in wide_bus: u64,
```

Ports are convenience syntax for temporal values; they can be modeled as library types like `Signal<Bit, Out>`.

## Internal Signals

Signals declared without direction are internal:

```ted
mod counter {
    in  clk: bit,
    out count: u8,

    // Internal signal
    let overflow: bit = 0;

    on rising(clk) {
        if count == 255 {
            overflow = 1;
        }
        count = count + 1;
    }
}
```

## Timed Contexts

`on` handlers and module-level `loop` blocks are timed contexts. `@` is only legal inside them; helper code outside remains ordinary core Ted.

> TODO: Add explicit timed task declarations and `timed fn` for timed code outside modules.

## Module Instantiation

Instantiate modules within other modules:

```ted
mod top {
    in  clk: bit,
    out led: bit,

    // Instantiate a counter
    counter my_counter {
        clk: clk,
        count: _, // unconnected
    };

    // Use counter output
    led = my_counter.overflow;
}
```

> TODO: Define the exact semantics of module-level assignments (for example, continuous assignment vs event-driven evaluation).

### Connection Syntax

```ted
module_type instance_name {
    port_name: signal,
    another_port: another_signal,
};
```

Use `_` for unconnected ports.

## Parameterized Modules

Modules can have parameters:

```ted
mod counter<WIDTH: u32 = 8> {
    in  clk: bit,
    out count: uint<WIDTH>,

    on rising(clk) {
        count = count + 1;
    }
}

// Instantiate with custom width
counter<16> wide_counter { ... };
```

## Module Hierarchy

Modules can contain other modules:

```ted
mod system {
    in clk: bit,

    // Connect them
    let bus: u32;
    cpu processor { clk: clk, data_out: bus };
    memory ram { clk: clk, data_in: bus };
}
```

## Visibility

By default, all ports are visible. Use `_` prefix for internal implementation details:

```ted
mod example {
    out result: u8,      // Public
    let _temp: u8,       // Private (convention)
}
```
