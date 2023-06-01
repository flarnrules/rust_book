# Ownership

Ownership is a discipline for memory safety.

Variables live in **frames**. Frames are mapping sfrom variables to values within a single scope, such as a function.

Ownership relates to memory, which means we need to understand **the stack** and **the heap**

A primary distinction between the two is that the stack holds data associated with a specific function, whereas the heap holds data that can outlive a function.
- frames in the stack are associated with a specific function, and are deallocated when the function returns.
- data on the heap can live indefinitely
- both stack and heap dta can be mutable and copyable
- the heap is allowed to contain pointers to the stack.