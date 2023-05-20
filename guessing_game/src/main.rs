use std::io; //bring the io (input/output) library into scope, the io library comes from the standard library known as std
use rand::Rng; //bring rand crate into scope and the Rng trait, which defines methods that random number genrators implement.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // call the rand::thread_rng function that gives the random number we wnt to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the gen_range method on the generator. This method is defined by the Rng trait brought into scope with `use rand::Rng;` statement. The `gen_range` method takes a range expression as an argument and generates a random number in the range with a form of `start..=end`

    println!("The secret number is: {secret_number}");
    
    println!("Please input your guess.");

    let mut guess = String::new(); // let statement creates a variable, the = sign tells Rust we want to bind something to the variable right now. the :: syntax indicates that new is an associated function of the `string` type. an associated function is a function that is implemented on a type. this `new` function creates a new, empty string. this line created a mutalble variable that is currently bound to a new, empty instance of a string.

    io::stdin() // calls the `stdin function from the io module, which allows us to handle user inptu
        .read_line(&mut guess) // this calls the read_line method to get input from the user. &mut guess is passed as the argument to read_line to tell what string to store the user input in. the & indicates that this argument being passed is a "reference" which gives you a way to let mulitiple parts of your code access one piece of data without the need to cpy the data into memory multiple times. references are mutable by default, hence the need to add mut.
        .expect("Failed to read line"); // calls expect in case the result of the .read_line method is "err". This is error handling.

    println!("You guessed: {guess}"); // this prints a string that contains the user's input. the curly brackets is a placeholder with our guess variable. When user iputs a value, that value binds to guess and is returned and appended to the string printed by this line of code.
}
