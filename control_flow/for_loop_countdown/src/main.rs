fn main() {
    for number in (1..4).rev() { // (1..4) is a range where the last number is omitted. The .rev method reverses the order of the loop.
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
