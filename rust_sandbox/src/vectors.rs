pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Primarily same operations as arrays.
    println!("{:?}", numbers);

    // Add values to vectors
    numbers.push(5);
    numbers.push(6);

    // Remove values to vectors
    numbers.pop();

    // Get single value
    println!("Get single value: {}", numbers[0]);

    // Re-assign value
    println!("Get original value of index 3: {}", numbers[2]);
    numbers[2] = 20;
    println!("Get single value of index 3: {}", numbers[2]);

    // Get array memory
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vectors values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vector: {:?}", numbers);
}
