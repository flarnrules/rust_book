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

**If Expressions**
If *expressions* use the `if` keyword to test a conditional and execute the following block of code if the conditional is true. If the conditional is false, the code will move on to the next block of code. We can string if *expressions* together with `else` and `else if`.

**Loops**
Loops repeat a block of code. There are three types of loops:

1. loop
2. while
3. for

**Loop Labels**
Loop labels can be used to help disambiguate loops, especially in situations where there's loops inside of loops. A loop label can be applied by starting with a single quote and then, in snake_case, writing a descriptive name of the loop followed by a colon, like this:

```rs
'loop_label: loop
```

## Ownership

Ownership rules in Rust are important concepts unique to rust, and they relate to how memory is allocated to either the stack or the heap.

**The Stack**
The stack is a form of storing data where each new addition of data is called *pushing onto the stack* and each removal of data is called *popping off the stack*. Every time new data is added or *pushed onto the stack*, it gets added to the top of the stack, like a stack of plates. Likewise, each time we *pop* something off of the stack, it is the top item. Thus it follows a *last in, first out* method.

**The Heap**
The heap is a form of storing data where it's less organized than the stack. You request a certain amount of space, and the memory allocator finds an empty spot that is big enough, marks it as being used and returns a *pointer*. This process is called *allocating on the heap* or just *allocating*. Adding data to the stack is ***not*** called allocating. The *pointer* itself can be stored on the stack because it is a known fixed size, but the data it points to is not on the stack, it's on the heap.

There are three ownership rules in rust:

1. Each value has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped.


## Borrowing

##
