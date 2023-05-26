fn main() {
    let mut count = 0; // initialize a mutable variable count with initial value of 0
    'counting_up: loop { // apply a loop label of "counting_up" to this loop
        println!("count = {count}"); // print the value for count
        let mut remaining = 10; // initialize a mutable variable remaining with initial value of 10

        loop { // second loop expression, inside the first one
            println!("remaining = {remaining}"); // print the value of the remaining variable
            if remaining == 9 { // if the remaining variable equals 9
                break; // end the inner loop
            }
            if count == 2 { // if the count variable equals 2
                break 'counting_up; // break the counting loop
            }
            remaining -= 1; // subtract 1 from the remaining variable. So when this second loop is entered, it will go here, and subtract 1 from the remaining variable. Then it will run the loop again and see that the remaining  variable is 9 and end the inner loop. how does the second if expression get used if we break the inner loop at 9? Oh wait, breaking the loop sends us back to the outer loop, which runs again. Hm. I still am not quite sure how we check count against 2. I understand that count goes to 2... oh I see. This loop runs the first time and subtracts 1 from remaining. Then it runs a second time and breaks... wait... I'm confused... okay. Let's try again

            // First time this loop runs, it subtracts 1 from remaining.
            // second time this loop runs, it sees remaining == 9 and breaks the inner loop.
            // the outer loop continues and adds 1 to count.
            // the outer loop then runs again, and repeates the process.
            // eventually count becomes 2, and when the inner loop runs, it sees that count == 2 and our "counting_up" labelled loop breaks, ending the full sequence before subtracting 1 from remaining.
        }

        count += 1; // add 1 to count once done with the inner loop.
    }
    println!("End count = {count}");

}
