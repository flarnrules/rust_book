use rand::Rng;
use std::cmp::Ordering;
use std::io;

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
    
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {guess}");
  
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
