// From Let's Get Rusty Chapter 3: https://youtu.be/2V0JaMVjzws
fn main() {

    // Variables are immutable. To make it mutable, use `mut` after `let`
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 69;
    println!("The value of x after mutation is {}", x);

    // Constants CAN'T be mutable AND they HAVE to be ANNOTATED
    // They also CAN't be set as the return value of a function
    // or any value that is computed at runtime
    const DOGE_VALUE: u32 = 1000000; 

    // Shadowing: Create a new variable using an existing name
    let x = "forty two";
    println!("The shadowed value of x is {}", x);


    // Scalar datatypes: single values
    // - Integers
    // - Floating-point numbers
    // - Booleans
    // - Character
    
    // Compound datatypes: group of values
    // - Tuple: Fixed size array of related data that can be of different types: 
    let tup = ("Bro", 69420);
    //  -> Also supports unpacking like Python! NOTE: Unused variables use the prefix _ by convention
    let (_nice_string, _nice_num) = tup;
    //  -> AND it supports accessing values by index, through property-like access
    let _nice_num = tup.1;

    // - Arrays: Fixed length, comma separated lists:
    let array = [200, 404];
    let _not_found = array[1];
    let _x = array[3]; // should return an error when running `cargo run`
    //  - Can also be created in the notation `let array = [0; 8];` which means
    //     create an array of size 8 and fill it with zeros
    let _zeros_array = [0; 8];
    
}
