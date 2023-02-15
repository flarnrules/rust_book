fn main() {
    println!("Hello, world!");

    let x = 2.0; // f64, when you don't provide a type for a float, default is f64

    let y: f32 = 3.0; // f32

    println!("The x variable is {}", {x});

    println!("The y varible is {}", {y});

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    
    // remainder
    let remainder = 43 % 5;

    println!("The mathematical operators above result in the folling values {}, {}, {}, {}, {}, {}.", {sum}, {difference}, {product}, {quotient}, {floored}, {remainder} );
}


