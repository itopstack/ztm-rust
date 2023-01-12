// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Drink {
    Coffee,
    Tea
}

struct Item {
    drink: Drink,
    fluid_ounce: f64
}

fn show_item(item: Item) {
    match item.drink {
        Drink::Coffee => println!("Coffee!"),
        Drink::Tea => println!("Tea!")
    }
    println!("{:?}", item.fluid_ounce);
}

fn main() {
    let item = Item {
        drink: Drink::Coffee,
        fluid_ounce: 10.5
    };
    show_item(item);
}
