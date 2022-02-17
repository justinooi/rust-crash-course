pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;

    // Simple if-else
    if age >= 21 && check_id {
        println!("Bartender: Legal");
    } else if age < 21 && check_id {
        println!("You're {}, too young.", age);
    } else {
        println!("ID pls.")
    }

    // Shorthand if

    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age)
}
