# Types

Ted has a simple, hardware-oriented type system.

## Primitive Types

### Bit

Single binary value (0 or 1):

```ted
let flag: bit = 0;
let enable: bit = 1;
```

### Unsigned Integers

Fixed-width unsigned integers:

| Type | Width | Range |
|------|-------|-------|
| `u8` | 8 bits | 0 to 255 |
| `u16` | 16 bits | 0 to 65,535 |
| `u32` | 32 bits | 0 to 4,294,967,295 |
| `u64` | 64 bits | 0 to 18,446,744,073,709,551,615 |

```ted
let byte: u8 = 42;
let word: u32 = 0xDEADBEEF;
```

### Signed Integers

Fixed-width signed integers (two's complement):

| Type | Width | Range |
|------|-------|-------|
| `i8` | 8 bits | -128 to 127 |
| `i16` | 16 bits | -32,768 to 32,767 |
| `i32` | 32 bits | -2^31 to 2^31-1 |
| `i64` | 64 bits | -2^63 to 2^63-1 |

```ted
let offset: i16 = -100;
```

### Custom Width

For arbitrary bit widths:

```ted
let nibble: uint<4> = 0xF;
let wide: uint<128> = 0;
```

## Arrays

Fixed-size arrays:

```ted
let memory: [u8; 256];           // 256 bytes
let registers: [u32; 16];        // 16 registers

// Access
let value = memory[0];
memory[index] = data;
```

## Structs

Group related signals:

```ted
struct Packet {
    valid: bit,
    data: u64,
    tag: u8,
}

let pkt: Packet;
pkt.valid = 1;
pkt.data = payload;
```

## Type Inference

Types can often be inferred:

```ted
let x = 42;           // inferred as u32
let y = x + 1;        // also u32
let flag = true;      // inferred as bit
```

## Type Conversions

### Explicit Casts

```ted
let narrow: u8 = wide as u8;      // truncate
let wider: u32 = narrow as u32;   // zero-extend
```

### Bit Extraction

```ted
let byte = word[7:0];             // bits 7 down to 0
let nibble = byte[3:0];           // lower nibble
```

### Concatenation

```ted
let combined = {high, low};       // concatenate
let word = {byte, byte, byte, byte};
```

## Constants

Compile-time constants:

```ted
const WIDTH: u32 = 8;
const DEPTH: u32 = 256;
const INIT: u8 = 0xFF;
```

## Literals

### Integer Literals

```ted
42          // decimal
0xFF        // hexadecimal
0b1010      // binary
0o77        // octal
1_000_000   // underscores for readability
```

### Boolean Literals

```ted
true        // same as 1 for bit
false       // same as 0 for bit
```

### Time Literals

```ted
10ns        // nanoseconds
1us         // microseconds
100ms       // milliseconds
1s          // seconds
```
