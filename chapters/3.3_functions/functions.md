# Functions

Functions are prevalent in Rust. The `main` function is the *entry point* of many programs. `fn` is the keyword we use to declare new functions.

Function and variable names are all in `snake_case` which means all lower case with underscores separating words.

We define a function by entering `fn` followed by a function name and a set of parenthesis:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

```

We can call any function that we have defined by entering its name followed by a set of parenthesis.

The curly braces {} define the beginning and ending of a function body.

In the above code the function `another_function` is defined in the program, thus it can be called inside the `main` function.

>❕Note
>
>It doesn't matter that `another_function` was defined after the main function

The code lines will execute in the order that they show up inside the main function.

## Parameters

We can define functions to have *parameters* or *params* in coderspeak. Parameters are special variables that are a part of a function's signature. When a function has parameters, you can provide it with concrete values of those parameters. These concrete values are called *arguments*, but casually the words *parameter* and *argument* are interchangebale.

Below is an example of a function with a parameter:

```rust

fn main() {
    another_function(5); // pass 5 to the function as an argument
}

fn another_function(x: i32) { // define function with one parameter of type 32 bit signed integer.
    println!("The value of x is: {x}");
}
```

> ⚠️ Alert!
>
> In function signatures, you *must* declare the type of each parameter. This is a deliberate decision in Rust's design: requiring type annotations in function definitions means the compiler is unlikely to need you to use them elsewhere in the code to figure out what type you mean. This also increases the ability of the compiler to give better error messages.

A function can have more than one parameter. Multiple parameters are separated by commas:

```rust
fn main() {
    print_labeled_measurements(5, 'h'); // char types are formatted with single quotes
}

// params value and unit lable are 32-bit signed integer and character types respectively
fn print_labeled_measurements(value: i32, unit_label: char) { 
    println!("The measurement is: {value}{unit_label}");
} 
```
So above we have `print_labeled_measurements` which has the params of `value` and `unit_label` of types i32 and char.

## Statements and Expressions

Function bodies are made up as a series of *statements* optionally ending in an *expression*. Rust is an *expression-based* language.

Other languages don't really have the same distinctions.

> ✔️ STATEMENTS
>
> *Statements* are instructions that perform some action and do NOT return a value.
>
>> For example creating a variable and assigning a value:
>> ```rust
>> fn main() {
>>       let y = 6;  
>> }
>> ```

> ✔️ EXPRESSIONS
>
> *Expressions* are statements that evaluate to a value. Expressions do not include ending semicolons.
>
>> For example, `x + 1` below doesn't end in a semicolon:
>> ```rust
>> fn main() {
>>     let y = {
>>         let x = 3;
>>         x + 1
>>     };
>>
>>     println!("The value of y is: {y}");   
>> }
>>

Adding a semicolon to the end of an expression turns it into a statement, and then will not return a value. This is a very important concept. Understanding **statements**, **expressions**, how semicolons work, and **return values** is fundamental to understanding Rust syntax 🦀.

## Functions with Return Values

The syntax `->` tells Rust to return a value. I need to read this section a few more times, in conjunction with the previous section. 🧠

