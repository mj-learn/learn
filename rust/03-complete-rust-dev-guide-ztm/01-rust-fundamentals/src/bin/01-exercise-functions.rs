// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name(first_name: &str) -> &str {
    first_name
}

fn last_name(last_name: &str) -> &str {
    last_name
}

fn main() {
    println!("My name is: {} {}", first_name("MOn"), last_name("Jofn"));
}
