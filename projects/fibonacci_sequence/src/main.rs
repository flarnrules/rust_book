use std::io;

fn fib_seq (n: i64) -> i64 {
    if n <= 0 {
        panic!("Index was non-positive");
    }
    else if n == 1 {
        return 0;
    }
    else if n == 2 {
        return 1;
    } else {
        return fib_seq(n - 1) + fib_seq(n - 2);
    }
}

fn main() {
    println!("Please provide the fibonacci number you would like to generate, between 1 and 45");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    
    let n: i64 = input.trim().parse()
        .expect("Please type a number!");

    println!("The {}th Fibonacci number is: {}", n, fib_seq(n));
}
