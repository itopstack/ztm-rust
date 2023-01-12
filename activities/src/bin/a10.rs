// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let n = 101;
    let less_than_100 = if n < 100 {
        true
    } else {
        false
    };
    print_result(less_than_100);
}

fn print_result(less_than_100: bool) {
    match less_than_100 {
        true => println!("It's small"),
        false => println!("It's big")
    }
}
