# Syntax Overview

Ted uses a Rust-like syntax that should feel familiar to systems programmers.

## Basic Structure

A Ted program consists of modules:

```ted
mod my_module {
    // port declarations
    in  input_signal: bit,
    out output_signal: bit,

    // logic
    on change(input_signal) {
        output_signal = input_signal;
    }
}
```

## Comments

```ted
// Single-line comment

/*
   Multi-line
   comment
*/
```

## Identifiers

Identifiers follow Rust conventions:
- Start with a letter or underscore
- Contain letters, digits, or underscores
- Case-sensitive

```ted
signal_name
_private
counter123
```

## Keywords

```
mod     in      out     inout
on      rising  falling change
if      else    loop    break
let     fn      return
bit     u8      u16     u32     u64
i8      i16     i32     i64
```

## Operators

### Arithmetic
```ted
a + b    // addition
a - b    // subtraction
a * b    // multiplication
a / b    // division
a % b    // modulo
```

### Bitwise
```ted
a & b    // AND
a | b    // OR
a ^ b    // XOR
~a       // NOT
a << n   // left shift
a >> n   // right shift
```

### Comparison
```ted
a == b   // equal
a != b   // not equal
a < b    // less than
a <= b   // less or equal
a > b    // greater than
a >= b   // greater or equal
```

### Logical
```ted
a && b   // logical AND
a || b   // logical OR
!a       // logical NOT
```

### Time
```ted
x @ -1       // past reference
x @ +10ns    // future scheduling
```

## Blocks

Blocks use curly braces:

```ted
{
    let x = 1;
    let y = 2;
    x + y
}
```

## Statements vs Expressions

Most constructs are expressions that return values:

```ted
let result = if condition { a } else { b };
```
