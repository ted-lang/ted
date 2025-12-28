# Example Programs

This section contains complete Ted example programs.

## Basic Examples

### Hello World

The simplest Ted program:

```ted
mod hello {
    out value: u8,
    value = 42;
}
```

### Blink

Toggle an LED every 500ms:

```ted
mod blink {
    out led: bit,

    loop {
        led = !led @ +500ms;
    }
}
```

### Counter

8-bit counter with reset:

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

## Intermediate Examples

### Edge Detector

Detect rising and falling edges:

```ted
mod edge_detector {
    in  signal: bit,
    out rising: bit,
    out falling: bit,

    on change(signal) {
        let prev = signal @ -1;
        rising = signal && !prev;
        falling = !signal && prev;
    }
}
```

### Pulse Generator

Generate a pulse of specific width:

```ted
mod pulse_gen {
    in  trigger: bit,
    in  width: u32,      // pulse width in ns
    out pulse: bit,

    on rising(trigger) {
        pulse = 1;
        pulse = 0 @ +width;
    }
}
```

### Debouncer

Debounce a noisy input signal:

```ted
mod debouncer {
    in  noisy: bit,
    out clean: bit,

    let stable_count: u8 = 0;
    const THRESHOLD: u8 = 10;

    on change(noisy) {
        stable_count = 0;
    }

    loop {
        if stable_count < THRESHOLD {
            stable_count = stable_count + 1 @ +1ms;
        } else {
            clean = noisy;
        }
    }
}
```

## Advanced Examples

### FIFO Queue

Simple first-in-first-out buffer:

```ted
mod fifo<DEPTH: u32 = 16> {
    in  clk: bit,
    in  write_en: bit,
    in  read_en: bit,
    in  data_in: u8,
    out data_out: u8,
    out empty: bit,
    out full: bit,

    let buffer: [u8; DEPTH];
    let write_ptr: u32 = 0;
    let read_ptr: u32 = 0;
    let count: u32 = 0;

    empty = count == 0;
    full = count == DEPTH;

    on rising(clk) {
        if write_en && !full {
            buffer[write_ptr] = data_in;
            write_ptr = (write_ptr + 1) % DEPTH;
            count = count + 1;
        }

        if read_en && !empty {
            data_out = buffer[read_ptr];
            read_ptr = (read_ptr + 1) % DEPTH;
            count = count - 1;
        }
    }
}
```

### Shift Register

Configurable shift register:

```ted
mod shift_reg<WIDTH: u32 = 8> {
    in  clk: bit,
    in  data_in: bit,
    in  load: bit,
    in  parallel_in: uint<WIDTH>,
    out data_out: bit,
    out parallel_out: uint<WIDTH>,

    let reg: uint<WIDTH> = 0;

    data_out = reg[WIDTH - 1];
    parallel_out = reg;

    on rising(clk) {
        if load {
            reg = parallel_in;
        } else {
            reg = {reg[WIDTH-2:0], data_in};
        }
    }
}
```

## Running Examples

Check an example:
```bash
ted check examples/counter.ted
```

Compile an example:
```bash
ted compile examples/counter.ted -o counter.tedbc
```

Simulate (when implemented):
```bash
ted sim counter.tedbc --cycles 100
```
