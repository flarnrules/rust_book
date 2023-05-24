fn main() {
    let x = plus_one(5); // the x variable is a function call with the 5 as its parameter.

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // this will return the value x + 1 when the plus_one function is called.
}
