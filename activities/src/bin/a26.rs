// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
    println!("{:?}", dt.to_string());
}
