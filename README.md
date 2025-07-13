# Accurate Float
This is a custom type that combines the flexibility from floating point numbers with the accuracy of fixed point numbers.

This type is stored fully inside 32 bits with a fixed size. With that fixed size it's able to be stored on the stack for quick access. 

## Construction of the type
The 32 bits are split up into 3 regions:
- 1. Pointer
- 2. Sign
- 3. Value

```shell
  1.  2.              3.
00000 0 0000000000000000000000000
```

### Pointer
The pointer is a plain 5 bit counter which shows the place of the decimal on a binary level. This means that if the pointer reads 5, that the last 5 bits of the value part are considered the fraction part.

### Sign
The sign is a simple bit that shows if the number is negative or positive. 0 for positive, 1 for negative.

### Value
The value is composed of 2 different integer numbers which get seperated by the pointer. The number specifically isnt stored in an exponent so that the inaccuracy is removed.

## Examples
Here will be shown some examples from the decimal representation into the binary and vice versa.

### Binary to Decimal

Binary Number: `00011 0 00000000000000000001100110`
| Part    | Value |
|---------|-------|
| Pointer |     3 |
| Sign    |     + |
| Value   |  12.6 |

The pointer seperates the value `1100110` at the last 3 bits which turns it into `1100` and `110`

| Binary | Decimal |
|--------|---------|
|   1100 |      12 |
|    110 |       6 |

Binary Number: `00101 1 00000000000000001010111011`
| Part    | Value  |
|---------|--------|
| Pointer |      5 |
| Sign    |      - |
| Value   | -21.27 |

The pointer seperates the value `1010111011` at the last 3 bits which turns it into `10101` and `11011`

| Binary | Decimal |
|--------|---------|
|  10101 |      21 |
|  11011 |      27 |