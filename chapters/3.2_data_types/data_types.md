# Data Types
- Every value in rust is of a certain *data type*
- We will look at two data type subsets: **scalar** and **compound**
- In later chapters there's something called **generic** types, which are super confusing.

*Note:*
Rust is a *statically typed* language, which means that it must konw what type we want to use based on the value and how we use it.

## Scalar
A *scalar* type represents a single value. Rust has four primary scalar types:
1. Integers,
2. Floating-point numbers,
* Numeric operators,
3. Booleans,
4. Characters

### 1 Integer Types
An integer is a number without a factional component.



Integer types in Rust:

<div style="text-align: center;">

| Length  | Signed | Unsigned |
|   ---   |   ---  |    ---   |
| 8-bit   |   i8   |    u8    |
| 16-bit  |   i16  |   u16    |
| 32-bit  |   i32  |   u32    |
| 64-bit  |   i64  |   u64    |
| 128-bit |  i128  |   u128   |
| arch    | isize  |  usize   |

</div>

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

### * Numeric Operators

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

Note: Integer division rounds down to the nearest integer.

Each expression above uses a mathematical operator and evalutes to a single value, which is then bound to the variable. Rust Book Appendix B contains a list of all operators provided in Rust.

### 3 The Boolean Type

A Boolean type in Rust has two values: `true` and `false`

Booleans are one byte in size. A Boolean type in Rust is specified using `bool`. Example:

```rust

fn main() {
    let t = true;

    let f; bool = false; // with explicit type annotation
}
```

The main way to use Boolean values is through conditionals, such as `if` expressions. This is covered in the Control Flow section of this chapter.

### 4 The Character Type

Rust's `char` type is the language's most primitive alphabetic type. Here's some examples of declaring `char` values:

```rust

fn main() {
    let c = 'z';
    let z: char = 'Z'; // with explicity type annotation
    let heart_eyed_cat = '😻';
    let crab = '🦀';
}
```
Note: we specify `char` literals with single quotes. String literals are specified with double quotes. A `char` type is four bytes in size, and represents a Unicode Scalar Value, which means it can represent more than just ASCII.

Unicode Scalar Values include accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces. The concept of `char` is discussed in more detail in Chapter 8 - "Storing UTF-8 Encoded Text with Strings.

## Compound

*Compound types* can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

1. Tuples
2. Arrays

### Compound Type 1 - Tuple

The tuple type is a general way of grouping together a number of values with a variety fo types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrin kin size.  

tuples can be created by writing a comma-separated list of values inside parantehses. Each position in the tuple has a type, and theypes of the different values int he touble don't have to be the same. We've added optional type annotations in this example.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // See? 3 different types
}
```

The variable `tup` binds to the entire tuple, because a tuple is considered a *single*, *compound* element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value like this:

```rust
fn main() {
    let tup = (500, 6.4 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```
The above program first creates a tuple, and then binds it to the variable `tup`. Then it uses a pattern with `let` to take `tup` and turn it into three separate variables: `x`, `y`, and `z`. This is called *desctructuring*, because it braks the single tuple into three parts. Then the program prints the value of y.  

We can also access a tuple element driectly by using a period (.) followed by the index of the value we ant to access. Tuple index begins with zero, so if we have 3 items in the tuple and we wanted the third item, we need to use the index 2. For example:


```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.s;
}   
```

The above program creates the tuple `x` and then accesses each element using their respective indeces. As with most programming languages, the first index in a tuple is 0.  

The tuple without any values has a psecial name, *unit*. This value and its corresponding type are both written as () and represent an empty value or an empty return type. Expressions implicitly return the unit value of they don't return any other value.

> Remember, in Rust, expressions and statemetns are two fundamental concepts that represent different kinds of code structures with different meanings.
>
> A **statement** is a code structure that performs an action without returning a value. For example, assigning a value to a variable, or a loop statement that repeats a block of code when a condition remains true. Statements do not produce any output, and their primary purpose is to cause a side-effect, like changing the program's state.
>
> An **expression** is a code structure that evaluates to a value. In Rust *almost* everything is an expression, including literals, variables, function calls, and arithmetic operations. Expressions can be used as part of a statement or as part of another expression.
>
> The key difference is that expressions have values while statements do not.

### Compound Type 2 - Array

Another way to have a collection of multiple values is an *array*. Unlike tuples, every element of an array must have the **same** type. In rust, arrays **must** have a fixed length.  

We write an array as a comma-separated list inside square brackets:  

```rust
fn main () {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the **stack** rather than the **heap**. See diagram below.

![StackVersusHeap](https://github.com/flarnrules/images/blob/main/stack_vs_heap.png)

Arrays are also useful when you want to ensure you always have a fixed number of elements. It is not as flexible as the vector type. A vector is a similar collection type provided by the standard libriary that is growable and shrinkable. During times of uncertainty, probably best to use a vector.

Arrays are useful when you know the number of elements will not need to change. For example, names of the month in a program, you always know there are 12 months.

```rust
let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
```

You write an array's type using square brackets with the type of each element, a semicolon, and then the number of elements in the array like this:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
Why didn't we need to do this with the months example?

You can also initialize an array to contain the same value for each element by specifying the intiial value, followed by a semicolon, and then the length of the array in square brackets like this:

```rust
let a = [3; 5];
```
In this example, the type was not specified.

**Accessing Array Elements**

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elemetns of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5]; // declare the array

    let first = a[0]; // declare "first" variable, at index 0. We are assigning the value 1 to first
    let second = a[1]; // declare "second" variable, at index 1. We are assigning the value 2 to second.
}
```

**Invalid Array Element Access**

Let's see what happens when we try to access an element of an array that is past the end of the array.

```rust

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}