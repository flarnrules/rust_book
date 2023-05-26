use std::time::{Instant, Duration};

fn main() {
    let mut counter = 0;
    let start = Instant::now();

    
    loop {
        println!("again!");
        counter += 1;

        if Duration::from_secs(10) < start.elapsed() { // The loop will break after 10 seconds.
            break;
        }
    }

    let duration = start.elapsed();

    println!("Printed 'again!' {} times", counter);
    println!("The loop ran for {:.2} seconds", duration.as_secs_f64());
    println!("Average time per iteration: {:.2} microseconds", duration.as_micros() as f64 / counter as f64);

}
