# Ownership

Ownership is a discipline for memory safety.

**Safety**, at least with Rust, is the *absence of undefined behavior*.

Ownership is the idea that every variable has an owner... and every variable can only have own owner at a time.

Variables live in **frames**. Frames are mappings from variables to values within a single scope, such as a function.

Ownership relates to memory, which means we need to understand **the stack** and **the heap**

A primary distinction between the two is that the stack holds data associated with a specific function, whereas the heap holds data that can outlive a function.
- frames in the stack are associated with a specific function, and are deallocated when the function returns.
- data on the heap can live indefinitely
- both stack and heap dta can be mutable and copyable
- the heap is allowed to contain pointers to the stack.

## Memory Management

Memory management is the process of allocating memory and deallocating memory.

**Box deallocation principle**: If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

- variables live in the stack
- boxes live in the heap
- a box's owner manages deallocation
- collections use boxes

With the code snippet `variable.push_str("some other string");`, `.push_str` is a method that's specifically for the `String` type. It appends a string slice onto the end of a `String`. This helps us to understand the need for `String` type variables to live on the heap so that more memory can be allocated.