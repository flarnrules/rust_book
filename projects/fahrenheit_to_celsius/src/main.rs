use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0 // because type is floating point, need to add .0 to end of each number in the expression
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0 // same, because the type is a floating point, need to add .0 to end of each number in the expression
}

fn main() {
    println!("1. Convert Fahrenheit to Celsius");
    println!("2. Convert Celsius to Fahrenheit");
    println!("Type either 1 or 2 based on the conversion needed!");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("Enter the temperature to be converted:");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    match choice {
        1 => println!("The temperature in Celsius is: {}", fahrenheit_to_celsius(temp)),
        2 => println!("The temperature in Fahrenheit is: {}", celsius_to_fahrenheit(temp)),
        _ => println!("Invalid choice"),
    }

}
