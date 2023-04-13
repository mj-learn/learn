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
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{clamp, div, concat};

    #[test]
    fn check_clamp_1() {
        let result = clamp(5, 3, 9);
        let exepected = 5;
        assert_eq!(result, exepected, "Error");
    }

    #[test]
    fn check_clamp_2() {
        let result = clamp(5, 5, 5);
        let exepected = 5;
        assert_eq!(result, exepected, "Error");
    }

    #[test]
    fn check_div_1() {
        let result = div(8, 4);
        let exepected = Some(2);
        assert_eq!(result, exepected, "Error");
    }

    #[test]
    fn check_div_2() {
        let result = div(15, 0);
        let exepected = None;
        assert_eq!(result, exepected, "Error");
    }

    #[test]
    fn check_concat_1() {
        let result = concat("Hello", "world");
        let exepected = String::from("Hello world");
        assert_eq!(result, exepected, "Error");
    }

    #[test]
    fn check_concat_2() {
        let result = concat("My name is", "MOnjofn");
        let exepected = String::from("My name is MOnjofn");
        assert_eq!(result, exepected, "Error");
    }
}
