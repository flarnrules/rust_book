fn main() {
    println!("Hello, world!");

    print_labeled_measurements(5, 'h');
}

fn print_labeled_measurements (value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
