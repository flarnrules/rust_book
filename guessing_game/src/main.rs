use std::io; //bring the io (input/output) library into scope, the io library comes from the standard library known as std
use rand::Rng; //bring rand crate into scope and the Rng trait, which defines methods that random number genrators implement.
use std::cmp::Ordering; // bring type `std::cmp::Ordering` into scope from the standard library. The `Ordering` type is an enum with variants `Less`, `Greater`, and `Equal`.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // call the rand::thread_rng function that gives the random number we wnt to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the gen_range method on the generator. This method is defined by the Rng trait brought into scope with `use rand::Rng;` statement. The `gen_range` method takes a range expression as an argument and generates a random number in the range with a form of `start..=end`

    // we will comment out the below line to make the game a game!
    // println!("The secret number is: {secret_number}");

    loop { // loops through everything contained inside the {}
        println!("Please input your guess.");

        let mut guess = String::new(); // let statement creates a variable, the = sign tells Rust we want to bind something to the variable right now. the :: syntax indicates that new is an associated function of the `string` type. an associated function is a function that is implemented on a type. this `new` function creates a new, empty string. this line created a mutalble variable that is currently bound to a new, empty instance of a string.

        io::stdin() // calls the `stdin function from the io module, which allows us to handle user inptu
            .read_line(&mut guess) // this calls the read_line method to get input from the user. &mut guess is passed as the argument to read_line to tell what string to store the user input in. the & indicates that this argument being passed is a "reference" which gives you a way to let mulitiple parts of your code access one piece of data without the need to cpy the data into memory multiple times. references are mutable by default, hence the need to add mut.
            .expect("Failed to read line"); // calls expect in case the result of the .read_line method is "err". This is error handling.

        // commented out the below line, and replaced it with a better way to handle invalid output:
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // using `let` a second time "shadows" the guess variable, allowing us to bind a different type to this variable. Above when we declared `guess` the value was stored as a type String, but now we are going to store it as a type u32, an integer that can be handled by the `cmp` method. trim().parse().expect() first modify the value in case it was entered incorrectly, and then provides an error response if the value is still not compaitble wit the u32 type.

        let guess: u32 = match guess.trim().parse() { // here we got rid of the .expect() method, and instead added some more error handling. we also added the match expression to move from crashing on error to handling the error. parse returns a `Result` type and `Result` is an enum with variants `Ok` and `Err`. The match expression is similar to the match expression below when comparing the guess to the secret number, but in this case it determines if the input can be parsed to the correct type (an integer)
            Ok(num) => num, // Ok result will return the num value of the guess, and then carry on with the rest of the program.
            Err(_) => continue, // Err result will match all Err values as the underscore `_` is a catchall value. If any error comes through, it will run the second part of the matche's arm `continue` which tells the program to go tot he next itration of the loop asking for another guess, and so on.
        };
        
        println!("You guessed: {guess}"); // this prints a string that contains the user's input. the curly brackets is a placeholder with our guess variable. When user iputs a value, that value binds to guess and is returned and appended to the string printed by this line of code.

        match guess.cmp(&secret_number) { // the `cmp` method compares two values and can be called on anything that cane be compared. In this case it's taking in a refrence to `secret_number` with the `&` symbol making secret_number a reference.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); // if guess equals secret number, print out the string telling the player they won.
                break; // also, end the game if the guess equals the scret number.
            }
        }
    }
}
