# Installing and Updating Rust



# Rust Key Concepts

## Cargo / version control

Running `$ cargo new <project name>` will generate a new project. Running `cargo new` will not generate Git files if run inside an existing Git repository. This can be overridden by running the command `cargo new --vcs=git <project name>`

By this, I'm pretty sure it means that it won't generate *new* Git files, which prevents conflicts between two git repos.

I don't understand how this works yet. After testing, running `cargo new --vcs=git <project name>` appears to generate a bunch of files inside my repo that are not connected to my Git repo and cannot be pushed to Github.

This behavior is essentially the opposite of what I would expect.


## Naming Conventions

*items typed in snake_case*
1. functions
2. variables

## Functions

**Define** function with `fn` followed by a function name and `()`

The function begins with `{` and ends with `}`

**Call** function with the function name folowed by `()`

Functions can be **called** from inside the main function.

## Types

Scalar types: 

- signed integers (i8, i16, i32, i64, i128)
- unsigned integers (u8, u16, u32, u64, u128)
- floating-point numbers (f32, f64) 
- numerical operators (+, -, *, /, %)
- Boolean (bool = true, false)
- Character (char = 'z' or char = 'Z')

Compound types:

- tuples - group a number of values with a variety of types into one compound type, fixed length:

```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- arrays - group number of values where every element has the same type, fixed length:

```rs
let array: [i32; 5] = [1, 2, 3, 4, 5]; // declares an array with 5 elements all i32 integers
```

Arrays can be initialized to contain the same value for each element like this:
```rs
let array = [3; 5];
```
Arrays allocate the data on the stack, rather than the heap and ensure you always have a fixed number of elements.
A vector is a similar collection type provided by the standard library that can grow or shrink.

Custom types:

Types of types:

## Statements and Expressions

A *statement* is an instruction that performs and action but does not return a value. Statements end in `;`

An *expression* is an instruction that evaluates to a value and does not end in a `;`

We can Use the `->` with a function to tell the function to return a value when the function is called.

## Control Flow

If *expressions* use the `if` keyword to test a conditional and execute the following block of code if the conditional is true. If the conditional is false, the code will move on to the next block of code. We can string if *expressions* together with `else` and `else if`.

Loops repeat a block of code. There are three types of loops:


## Ownership

## Borrowing

##
