use std::io;

fn main() {
    // let a = [1, 2, 3, 4, 5]; // initialize an array of 5 elements

    // let first = a[0]; // access the first element of an arry with index 0

    // let second = a[1]; // access the second element of an array with index 1

    let a = [1, 2, 3, 4, 5]; // initialize another array with 5 elements

    println!("Please enter an array index.");

    let mut index = String::new(); // define index as a variable, as a mutable String type

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
