fn main() {
    let mut counter = 0; // initializes a variable `counter` with a starting value of 0

    let result = loop {
        counter += 1; // add 1 each time the loop iterates

        if counter == 10 { // conditional expression that is true as soon as counter reaches 10, so after 10 loops, this expression will activate
            break counter * 2; // `counter * 2` is the expression that returns a value to be used once the loop breaks
        }
    };

    println!("The value of result is: {}", result);
}
