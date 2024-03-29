// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

// * Use a function that returns a tuple
fn coordinate(x:i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    // * Destructure the return value into two variables
    let (x, y) = coordinate(1, 6);
    
    // * Use an if..else if..else block to determine what to print
    if y == 5 {
        println!("y is equal to 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else {
        println!("y is greater than 5");
    }

    println!("x is {}", x);
}
