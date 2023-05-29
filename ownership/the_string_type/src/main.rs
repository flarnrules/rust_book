// the String type was not covered in the Data Types section of Chapter 3.
// all of the types covered in chapter 3 were of a known size and could be stored on the stack and popped off the stack when their scope was over.
// because these types are stored on the stack, they can be quikcly and trivially copied to make new, independent instances if your code needs to use the same value but in a different scope

fn main() {
    println!("Hello, world!");
}
