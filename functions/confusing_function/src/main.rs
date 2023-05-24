fn f(x: i32) -> i32 { x + 1 } // defines a function `f`

fn main() {
    println!("{}", f({
        let y = 1;
        y + 1
    }));
}
