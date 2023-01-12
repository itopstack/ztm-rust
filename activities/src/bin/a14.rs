// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    color: String,
    name: String,
    age: i32
}

impl Person {
    fn print_name(&self) {
        println!("{:?}", self.name);
    }

    fn print_color(&self) {
        println!("{:?}", self.color);
    }
}

fn main() {
    let persons = vec![
        Person {
            color: "Red".to_owned(),
            name: "Bob".to_owned(),
            age: 8
        },
        Person {
            color: "Green".to_owned(),
            name: "Alice".to_owned(),
            age: 6
        },
        Person {
            color: "Blue".to_owned(),
            name: "Mark".to_owned(),
            age: 24
        }
    ];

    for person in persons {
        if person.age <= 10 {
            person.print_color();
            person.print_name()
        }
    }
}
