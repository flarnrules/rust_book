use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // String is a string type that is growable

    io::stdin()
        .read_line(&mut guess) // Puts whatever the user enters into the string we pass to it, but also returns a `Result` value
        // Result is an enumeration (enum for short), which is a type that can be in one of multiple possible states. Each possible state is called a variant.
        .expect("Failed to read line"); // It's good to introduce a new line when you call a method with .method_name() syntax
    
    println!("You guessed: {guess}");
}
