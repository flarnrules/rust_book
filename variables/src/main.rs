fn main() {
    // let mut x = 5;
    
    // println!("The value of x is: {x}");
    
    // x = 6;
    
    // println!("The value of x is: {x}");

    let x = 5; // declare a variable x and bind the value 5 to it

    let x = x + 1; // decalre a new variable by shadowing the variable x and bind x + 1 to it, resulting in 6

    {
        let x = x * 2; // declare a new variable by shadowing the variable x again and bind x * 6, resulting in 12, also in a new scope
        println!("The value of x is: {x}"); // print this line to demonstrate different scopes of the variables
    }

    println!("The value of x is: {x}"); // print this line outside the previous scope to demonstrate how scoped shadowing works

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let spaces = "   "; // first we declare the variable spaces, and bind the string of three blank spaces
    let spaces = spaces.len(); // then we shadow the first variable with a new spaces variable, and bind a value that counts the number of spaces which defaults to a usize type this allowed us to declare a new variable, same name, different type.
    println!("The number of spaces is: {spaces}");

    
}
