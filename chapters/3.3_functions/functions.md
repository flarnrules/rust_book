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

