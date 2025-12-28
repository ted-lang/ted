# Modules

Modules are the basic building blocks of Ted programs.

## Module Declaration

```ted
mod module_name {
    // ports
    // internal signals
    // logic
}
```

## Ports

Ports define the module's interface:

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

    // Sub-modules
    cpu processor { clk: clk };
    memory ram { clk: clk };

    // Connect them
    processor.data_out -> ram.data_in;
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
