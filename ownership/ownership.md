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

Ownership is primarily a discipline of heap management:

- All heap data must be owned by exaclty one variable
- rust deallocates heap data once its owner goes out of scope
- ownership can be transferred by moves, which happen on assignments and function calls
- heap data can only be accessed through its current owner, not a previous owner

## References are non-owning pointers

A **reference** is a kind of pointer. We reference a variable with `&`. When we reference a variable, we are borrowing that variable.

References are thus *non-owning pointers* in that they do not own the data they reference, they merely borrow it.

The Rust ensures the safety of rererences through the **borrow checker**

Variables have three kinds of *permissions* on their data:

- Read (R) - data can be copied to another location
- Write (W) - data can be mutated in-place
- Own (O) - data can be moved or dropped.

These permissions only exist within the compiler.

By default, a variable has read/own permissions on its data (RO)

If a variable is annoted with `let mut` then it also has the write permission (W)

References can temporarily remove these permissions.

More generally, permissions are defined on **paths** and not just variables.

A path is anything you can put on the left-hand side of an assignment:

- Variables, like `a`
- Dereferences of paths, like `*a`
- Array accesses of paths, like `a[0]`
- Fields of paths, like `a.0` for tuples or `a.field` for structs.
- Any combination of the above, like `*((*a)[0].1)`

Recall the *Pointer Safety Principle*: data should not be aliased and mutated.
So, when we create a reference to data (borrow it), that data is temporarily read-only until the reference is no longer in use.

Mutable references (also called unique references) temporarily provide mutable access to data without moving it. We use `&mut` before the variable name to make a mutable reference. When a reference is mutable, aliasing is prevented, and mutation is allowed.

Permissions are returned at the end of a reference's lifetime.

A reference's lifetime is the range of code spanning it's birth (when the reference is created with the `&` operator) through its death (the last time the reference is used). Usage of a reference can be something like the variable a reference is assigned to being reassigned to a different variable... I think. This is really complicated.


