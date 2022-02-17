pub fn run() {
    let mut count = 0;

    //Infinite loop
    loop {
        count += 1;
        println!("{}", count);

        // breaks loop
        if count == 20 {
            break;
        }
    }

    let mut count2 = 0;
    // While loop (fizzbuzz) 
    while count2 <= 100 {

        if count2 % 15 == 0 {
            println!("fizzbuzz")
        } else if count2 % 3 == 0 {
            println!("fizz")
        } else if count2 % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count2)
        }

        count2 += 1;
    }

    // For range loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz")
        } else if x % 3 == 0 {
            println!("fizz")
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x)
        }
    }
}