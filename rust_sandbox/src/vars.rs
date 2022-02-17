// Variables hold primitive data or reference to data.
// Variables are immutable by default.
// Rust is block-scoped language

pub fn run() {
    let name = "Brad";
    let age = 37;

    // Does not work because they are always immutable.
    // age = 38;

    println!("My name is {} and I am {} - immutable", name, age);

    let mut ageMut = 37;
    println!("My name is {} and I am {} - mutable", name, ageMut);

    ageMut = 38;
    println!("My name is {} and I am {} - mutable", name, ageMut);

    const ID: i32 = 001; // const must be statically typed.
    println!("ID: {}", ID);
    
    // Assign multiple variables at once.

    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}