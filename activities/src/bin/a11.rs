// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id_number: i32,
    quantity: i32
}

fn display_id(grocery: &Grocery) {
    println!("{:?}", grocery.id_number);
}

fn display_quantity(grocery: &Grocery) {
    println!("{:?}", grocery.quantity);
}

fn main() {
    let grocery = Grocery {
        id_number: 1,
        quantity: 10
    };
    display_id(&grocery);
    display_quantity(&grocery);
}
