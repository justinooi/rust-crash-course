//Traditional Structs
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple struct
struct TColor(u8, u8, u8);

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

// Implement function to struct
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn change_name(&mut self, last: &str) {
        self.last_name = String::from(last);
    }

    fn to_tuple(&self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn run() {
    let mut black = Color {
        red: 255,
        green: 255,
        blue: 255,
    };

    black.red = 200;

    println!("{}, {}, {}", black.red, black.green, black.blue);

    let mut t_color = TColor(255,255,255);

    t_color.0 = 200;

    println!("{}, {}, {}", t_color.0, t_color.1, t_color.2);

    let mut new_person = Person::new("John", "Doe");
    println!("{:?}", new_person);
    println!("{}", new_person.full_name());
    new_person.change_name("Meme");
    println!("{}", new_person.full_name());
    println!("{:?}", new_person.to_tuple());
}
