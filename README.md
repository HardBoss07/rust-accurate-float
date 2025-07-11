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