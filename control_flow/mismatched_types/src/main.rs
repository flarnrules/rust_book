fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" }; // this will cause an error because arm 1 is an i32 and arm 2 is a string type.

    println!("The value of number is: {number}");
}
