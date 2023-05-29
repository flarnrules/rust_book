// A scope is the range within a program for which an item is valid. we will explore scope below for a basic variable

fn main() { // s is not valid here, it's not yet declared, it is not within scope
    let s = "hello"; //s is valid from this point forward

    // do some stuff with s
} // this scope is now over, and s is no longer valid or within scope

// when s comes into scope, it is valid
// it remains valid until it goes out of scope

fn other_function () {
    let s = String::from("hello"); // s is valid from this point forward
    
    // do some stuff with s

} // this scope is now over, and s is now longer valid

// there is a natural point at which we can return the memory our String needs to the allocator:
// when `s` goes out of scope. Rust calls a special function called `drop` which is called
// automatically at the closing curly bracket.