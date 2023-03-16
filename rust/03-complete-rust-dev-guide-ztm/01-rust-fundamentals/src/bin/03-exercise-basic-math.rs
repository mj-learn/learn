// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// * Use a function to add two numbers together
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// * Use a function to display the result
fn print_sum(result: i32) {
    // * Use the "{:?}" token in the println macro to display the result
    println!("Sum is {:?}", result);
}

fn main() {
    let res = sum(4, 3);
    print_sum(res);
}
