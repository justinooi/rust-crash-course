pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "float64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45455454545454545;

    // Find max size of type.
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean.
    let is_active: bool = true;

    // Get boolean from expression.

    let is_greater = 10 > 5;

    println!("{:?}", (x, y, z, is_active, is_greater));
}
