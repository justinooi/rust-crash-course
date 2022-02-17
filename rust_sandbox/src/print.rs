pub fn run() {
    // Print to console.
    println!("Hello from print.rs file!");

    // String literals and placeholders.
    println!("Number: {}", 1);

    // Index arguments for formatting
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code.");

    // Named arguments for formatting
    println!("{name} likes to play {activity}", name="John", activity="baseball");

    // Placeholder traits
    println!("Binary: {:b} - Hex: {:x} - Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, "tuple", true));
    
    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}