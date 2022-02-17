pub fn run() {
    // Fixed list array where elements are all of same types
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // Get single value
    println!("Get single value: {}", numbers[0]);

    // Re-assign value
    println!("Get original value of index 3: {}", numbers[2]);
    numbers[2] = 20;
    println!("Get single value of index 3: {}", numbers[2]);

    // Get array memory
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}