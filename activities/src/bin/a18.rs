// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    age: i32,
    name: String
}

impl Adult {
    fn new(age: i32, name: &str) -> Result<Self, String> {
        if age >= 21 {
            Ok(
                Self {
                    age: age, 
                    name: name.to_owned()
                }
            )
        } else {
            Err("Under 21 years old".to_owned())
        }
    }
}

fn main() {
    let person1 = Adult::new(21, "Bob");
    let person2 = Adult::new(18, "Alice");
    
    match person1 {
        Ok(adult) => println!("{:?}", adult),
        Err(e) => println!("{:?}", e)
    }

    match person2 {
        Ok(adult) => println!("{:?}", adult),
        Err(e) => println!("{:?}", e)
    }
}
