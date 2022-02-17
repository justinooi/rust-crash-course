use std::env;

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let x = args[1].clone().parse::<i32>().unwrap();
    let y = args[2].clone().parse::<i32>().unwrap();

    println!("{:?}", args);
    println!("{}", get_sum(x, y));
}