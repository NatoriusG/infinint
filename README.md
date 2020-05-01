# infinint
A semi-infinite-precision integer type, implemented in Rust.

## Examples

```rust
let a = Infinint::new();
assert_eq!(a.negative(), false);
assert_eq!(a.digits(), [0]);

let b = Infinint::from(123_456);
assert_eq!(b.digits(), [6, 5, 4, 3, 2, 1]);
```

## Implementation

The `Infinint` struct contains two elements: a boolean to identify the integer as positive or
negative, and a vector of bytes containing the decimal digits. Each byte in the vector holds
data for two decimal digits: one in the upper nybble, and one in the lower. The decimal digits
are stored in little-endian order.

For example, the decimal number `1998` would be stored as the following two bytes:
```lang-none
[1000_1001, 1001_0001]
```

which represent these decimal digit pairs:
```lang-none
[(8, 9), (9, 1)]
```

If the number of decimal digits is uneven, the lower nybble of the final byte will be 0. For
example:
```lang-none
137 = [0111_0011, 0001_0000] = [(7, 3), (1, 0)]
```
