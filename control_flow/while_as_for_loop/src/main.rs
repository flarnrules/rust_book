fn main() {
    let a = [10, 20, 30, 40, 50]; // initialize a variable `a` as a array with 5 values
    let mut index = 0; // initialize a mutable variable `index` with the value of 0.

    while index < 5 { // while conditional checks to see if index is less than 5. Loop continues until index is 5.
        println!("the value is: {}", a[index]); // prints the line with a value a[index], looks up the index in the array to return the value.

        index += 1; // adds 1 to the index to loop through each index in the array.
    }
}

// there is a much more efficient way to run this code. See for_loops for an example.
