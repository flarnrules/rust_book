# Data Types
- Every value in rust is of a certain *data type*
- We will look at two data type subsets: **scalar** and **compound**

Note:
Rust is a *statically typed* language, which means that it must konw what type we want to use based on the value and how we use it.

## Scalar
A *scalar* type represents a single value. Rust has four primary scalar types:
1. Integers,
2. Floating-point numbers,
4. Numeric operators,
5. Booleans,
6. characters

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

### 2 Floating-Point Types

There are two primitive types for *floating-point numbers*. Floating-point numbers are numbers with decimal points.

Example:

```rust

fn main() {
    let x = 2.0; // f64 - the type was not explicitly declared, but because f64 is the default type, Rust sets x as an f64.

    let y: f32 = 3.0 // f32
}
```

Floating-point integers default to f64 because on modern CPUs, a 64 bit piece of data is roughly the same speed as a 32 bit piece of data, but it's capable of more precision.

Floating-point numbers are represented according to IEEE-754 standard. The `f32` type is a single-precision float, and the `f64` has double precision.

### 3 Numeric Operators

Addition, subtraction, multiplication, division, and remainder.

``` rust

fn main(){
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 10-5;

    // multiplication
    let product = 5 * 10;

    // division
    let quotient = 1000 / 10;
    let floored = 10 / 1000;

    // remainder
    let remainder = 43 % 5; // also known as "modulus"

}
```

Note: Integer division round sown to the nearest integer.

## Compound
