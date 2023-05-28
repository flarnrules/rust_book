const DAYS_OF_CHRISTMAS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

const GIFTS_OF_CHRISTMAS: [&str; 12] = [
    "and a Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "Five Gold Rings",
    "Six Geese a Laying",
    "Seven Swans a Swimming",
    "Eight Maids a Milking",
    "Nine Ladies Dancing",
    "Ten Lords a Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming",
];

const FIRST_DAY_GIFT: &str = "A Partridge in a Pear Tree";

fn main() {
    for (day_number, day) in DAYS_OF_CHRISTMAS.iter().enumerate() {
        println!("On the {} day of Christmas my true love gave to me:", day);

        if day_number == 0 {
            println!("{}", FIRST_DAY_GIFT);
        } else {
            for gift_number in (0..=day_number).rev() {
                println!("{}", GIFTS_OF_CHRISTMAS[gift_number]);
            }
        }
        println!();  // This will print a blank line
    }
}
