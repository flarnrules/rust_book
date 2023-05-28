use std::io;

fn fib_seq(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => {
            let (mut a, mut b, mut i) = (0, 1, 2);
            while i < n {
                let c = a + b;
                a = b;
                b = c;
                i += 1;
            }
            b
        }
    }
}


fn main() {
    println!("Please provide the fibonacci number you would like to generate (between 1 and 187)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse()
        .expect("Please type a number!");

    println!("The {}th Fibonacci number is: {}", n, fib_seq(n));
}
