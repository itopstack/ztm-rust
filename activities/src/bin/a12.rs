// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Blue
}

struct Box {
    dimensions: f64,
    weight: f64,
    color: Color
}

impl Box {
    fn new(dimensions: f64, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color
        }
    }

    fn print_characteristics(&self) {
        match self.color {
            Color::Red => println!("{:?}, {:?}, Red", self.dimensions, self.weight),
            Color::Blue => println!("{:?}, {:?}, Blue", self.dimensions, self.weight)
        }
    }
}

fn main() {
    let item = Box::new(10.0, 50.0, Color::Red);
    item.print_characteristics();
}
