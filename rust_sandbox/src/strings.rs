pub fn run() {
    
    // Immutable fixed-length string literal.
    let hello = "hello";

    // Heap-allocated growable string.
    let mut grow_hello = String::from(hello);
    println!("{}", grow_hello);

    // Get length
    println!("{}", grow_hello.len());

    // Pushing into string
    grow_hello.push_str(", world");
    println!("{}", grow_hello);
    
    // Change string data
    grow_hello = String::from("new string");
    println!("{}", grow_hello);
    grow_hello = String::from("hello, world");

    // Capacity in bytes
    println!("Capacity: {}", grow_hello.capacity());

    // is_empty string method
    println!("Is Empty: {}", grow_hello.is_empty());

    // Check for substrings.
    println!("Contains 'world: {}", grow_hello.contains("world"));

    // Replace string
    println!("Replace: {}", grow_hello.replace("world", "there"));

    // Loop through string by whitespace.
    for word in grow_hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity.
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(10, s.capacity());

    println!("{}", s);
}