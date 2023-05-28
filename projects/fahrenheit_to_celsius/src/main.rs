use std::io;

fn f_to_c (f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn c_to_f (c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn main () {
    println!("Please choose what conversion you need:");
    println!("Choice 1: Convert Fahrenheit to Celsius");
    println!("Choice 2: Convert Celsius to Fahrenheit");

    let mut conversion_type: Option<u32> = None;

    loop {

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().unwrap_or(0);
        

        match choice {
            1 => {println!("You chose 1, now enter the degrees in Fahrenheit to be converted.");conversion_type = Some(1); break;},
            2 => {println!("You chose 2, now enter the degrees in Celsius to be converted.");conversion_type = Some(2); break;},
            _ => {println!("That's an invalid choice, please just type \"1\" or \"2\"!"); continue;},
        }
    };

    if let Some(conversion_type) = conversion_type {
        loop {
            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            match temp.trim().parse::<f64>() {
                Ok(val) => {

                    match conversion_type {
                        1 => println!("You entered {} Fahrenheit. The corresponding temperature in Celsius is {}", val, f_to_c(val)),
                        2 => println!("You entered {} Celsius. The corresponding temperature in Fahrenheit is {}", val, c_to_f(val)),
                        _ => unreachable!(),
                    };
                    break;
                },
                Err(_) => {
                    println!("You need to enter a temperature, such as -25 or 424");
                },
            };
        }
    }
}











// use std::io;

// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     (f - 32.0) * 5.0/9.0 // because type is floating point, need to add .0 to end of each number in the expression
// }

// fn celsius_to_fahrenheit(c: f64) -> f64 {
//     (c * 9.0/5.0) + 32.0 // same, because the type is a floating point, need to add .0 to end of each number in the expression
// }

// fn main() {
//     println!("1. Convert Fahrenheit to Celsius");
//     println!("2. Convert Celsius to Fahrenheit");
//     println!("Type either 1 or 2 based on the conversion needed!");
//     let mut choice = String::new();
//     io::stdin()
//         .read_line(&mut choice)
//         .expect("Failed to read line");

//     let choice: u32 = match choice.trim().parse() {
//         Ok(num) => num,
//         Err(_) => return,
//     };

//     println!("Enter the temperature to be converted:");
//     let mut temp = String::new();
//     io::stdin()
//         .read_line(&mut temp)
//         .expect("Failed to read line");

//     let temp: f64 = match temp.trim().parse() {
//         Ok(num) => num,
//         Err(_) => return,
//     };

//     match choice {
//         1 => println!("The temperature in Celsius is: {}", fahrenheit_to_celsius(temp)),
//         2 => println!("The temperature in Fahrenheit is: {}", celsius_to_fahrenheit(temp)),
//         _ => println!("Invalid choice"),
//     }

// }
