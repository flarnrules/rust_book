fn main() {
    let mut number = 3; // declare and initialize a variable `number` with 3 as the starting value.

    while number != 0 { // while loop, the conditional is that number is not equal to zero. The loop will run until `number` equals zero.
        println!("{number}!"); // print the value stored in the number variable.

        number -= 1; // this loop subtracts one from the number variable on each iteration.
    }

    println!("LIFTOFF!!!"); // We don't need a `break` keyword anywhere in this while loop. Once the condition is met, the loop stops automatically.
}
