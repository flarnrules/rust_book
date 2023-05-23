fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // general way of grouping together multiple values with a variety of types into a compound type

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0; // we can access a tuple element directly using the period (.) after the tuple variable and the index of the value, starting with 0

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The values of five_hundred, six_point_four, and one are: {five_hundred}, {six_point_four}, {one}");


}
