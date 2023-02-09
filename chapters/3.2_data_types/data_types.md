# Data Types
- Every value in rust is of a certain *data type*
- We will look at two data type subsets: **scalar** and **compound**

Note:
Rust is a *statically typed* language, which means that it must konw what type we want to use based on the value and how we use it.

## Scalar
A *scalar* type represents a single value. Rust has four primary scalar types:
1. Integers,
2. Floating-point numbers,
3. Booleans,
3. characters

### 1 Integer Types
An integer is a number without a factional component.

Integer types in Rust:

| Length  | Signed | Unsigned |
|   ---   |   ---  |    ---   |
| 8-bit   |   i8   |    u8    |
| 16-bit  |   i16  |   u16    |
| 32-bit  |   i32  |   u32    |
| 64-bit  |   i64  |   u64    |
| 128-bit |  i128  |   u128   |
| arch    | isize  |  usize   |

Each integer can be either signed on unsigned and has an explicit size. Signed means it can be both positive or negative, unsigned just means it's a positive number.

Signed stores numbers from -(2<sup>n-1</sup>) to 2<sup>n-1</sup>-1 so the formula works like this for an i8 integer:

 -(2<sup>7</sup>) to (2<sup>7</sup>)-1 = -128 to 127.

Unsigned stores numbers from 0 to 2<sup>n</sup>-1 so the formual works like this for a u8 integer:

2<sup>8</sup>-1 = 0 to 255.

## Compound
