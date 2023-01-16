// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp_lower_bound() {
        let result = clamp(-1, 0, 99);
        let expected = 0;
        assert_eq!(result, expected, "n should be inbound");
    }

    #[test]
    fn test_clamp_upper_bound() {
        let result = clamp(100, 0, 99);
        let expected = 99;
        assert_eq!(result, expected, "n should be inbound");
    }

    #[test]
    fn test_clamp_middle() {
        let result = clamp(50, 0, 99);
        let expected = 50;
        assert_eq!(result, expected, "n should be inbound");
    }

    #[test]
    fn test_divide_some() {
        let result = div(10, 2);
        let expected = Some(5);
        assert_eq!(result, expected, "Numbers should be divisible");
    }

    #[test]
    fn test_divide_none() {
        let result = div(10, 0);
        let expected = None;
        assert_eq!(result, expected, "Numbers should not be divisible");
    }

    #[test]
    fn test_concat() {
        let result = concat("Hello", "World");
        let expected = "HelloWorld".to_owned();
        assert_eq!(result, expected, "str1 and str2 should be concated");
    }
}