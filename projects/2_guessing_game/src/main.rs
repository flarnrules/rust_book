use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses = 10;

    while guesses > 0 {
        println!("You have {} guesses remaining.", guesses);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            guesses -=1;

            println!("You guessed: {guess}");
            
            if guesses == 0 {
                println!("You lost. The secret number was {}", secret_number)
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
            
            guesses -=1;
        }
    
    if guesses == 0 {
        println!("You lost. The secret number was {}", secret_number);
    }
    }
}
