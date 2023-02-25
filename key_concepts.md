# Rust Key Concepts

## Cargo / version control

Running `$ cargo new <project name>` will generate a new project. Running `cargo new` will not generate Git files if run inside an existing Git repository. This can be overridden by running the command `cargo new --vcs=git <project name>`

By this, I'm pretty sure it means that it won't generate *new* Git files, which prevents conflicts between two git repos.

I don't understand how this works yet. After testing, running `cargo new --vcs=git <project name>` appears to generate a bunch of files inside my repo that are not connected to my Git repo and cannot be pushed to Github.

This behavior is essentially the opposite of what I would expect.


## When to use snake_case

*items typed in snake_case*
1. functions
2. variables

## Functions

**Define** function with `fn` followed by a function name and `()`

The function begins with `{` and ends with `}`

**Call** function with the function ame folowed by `()`

Functions can be **called** from inside the main function.

## Types

Scalar types:

Compound types:

Custom types:

Types of types:


