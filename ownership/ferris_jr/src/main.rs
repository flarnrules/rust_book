fn main() {
    let first = String::from("Ferris"); // L1
    let full = add_suffix(first); // L4
    println!("{full}"); // this uses a named argument, rather than a positional argument. It is more advanced. Slightly less verbose, but could be more confusing.
}

fn add_suffix(mut name: String) -> String { // L2
    name.push_str(" Jr."); // L3
    name
}

// L1: The string "Ferris" has been allocated on the heap. It is owned by first.
// L2: The function add_suffix(first) has been called. This moves ownership of the string from first to name. The string data is not copied, but the pointer to the data is copied. I believe this is called a "move"
// L3: the function name.push_str(" Jr.") resizes the string's heap allocation. This does three things:
    // 1: First, It creates a new larger heap allocation.
    // 2: Second, It writes "Ferris Jr." into the new allocation.
    // 3: Third, It frees the original heap memory. `first` now points to deallocated memory.
// L4: The frame for `add_suffix` is gone. This function returned name (hence no semicolon) and transferred ownership of the string to `full`