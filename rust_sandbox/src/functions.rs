fn greeting(greet: &str, name: &str) {
    println!("{}, {} - nice to meet you", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn run() {
    greeting("hello", "jane");
    let get_sum = add(30, 30);
    println!("{}", get_sum);

    // Closure
    let z = 5;
    let add_nums = |x: i32, y: i32| x + y + z;
    println!("{}", add_nums(30, 30));
}