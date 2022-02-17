pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("{:?}, {:?}", arr1, arr2);

    // Non-primitives, if you assign another variable to point to same data, first variable no longer valid.

    let vec1 = vec![1, 2, 3];
    // let vec2 = vec1;
    let vec2 = &vec1;

    // println!("{:?}, {:?}", vec1, vec2) - not possible
    println!("{:?}, {:?}", vec1, vec2)
}
