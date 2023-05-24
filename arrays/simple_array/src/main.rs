fn main() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]; // arrays are good for lists of thing that wont' change, like this list of months. There will always be 12 months.

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // here we intialize an array with 5 elements, each element is defined to the right of the '='

    let a = [3; 5]; // here we initialize an array with 5 elements, all elements are a 3.

    let a = [1, 2, 3, 4, 5]; // by using shadowing, we can replace the a array back to where we started.

    // let's access elements. an array is a single chunk of memory of a known, fixed size, that can be allocated on the "stack". More on the stack later.

    let first = a[0]; // here we define the variable "first" and then bind it to the first index in the array "a"

    let second = a[1]; // here we do the same but for the second index in array "a"

    // go to arrays -> invalid_element_access for an example of invalid access to a non-defined array element
}
