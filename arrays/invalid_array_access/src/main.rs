use std::io; // brings the std::io library into the scope of the program

fn main() { // this is the entry point into the program
    let a = [1, 2, 3, 4, 5]; // defines an array 'a' with values 1 through 5. default type of this is an i32.

    println!("Please enter an array index."); // prints the string to the console

    let mut index = String::new(); // declares the mutable variable `index` and initializes it to a new empty string

    io::stdin() // brings in the standard input method
        .read_line(&mut index) // reads the line from a standard input (keyboard in this case) into the index variable
        .expect("Failed to read line"); // prints an error message if the line is unsuccessfully read

    let index: usize = index
        .trim() // trim any leading or trailing whitespace
        .parse() // attempts to parse the index string as an unsigned integer type
        .expect("Index entered was not a number");

    let element = a[index]; // retrieves the element at the `index` position of the `a` array and assigns it to the `element` variable

    println!("The value of the elment at index {} is: {}", index, element);

}
