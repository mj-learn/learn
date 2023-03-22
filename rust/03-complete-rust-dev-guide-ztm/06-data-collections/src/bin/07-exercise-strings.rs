// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a people age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a people age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    name: String,
    age: i32,
    color: String,
}

// * The name and colors should be printed using a function
fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn print_color(color: &str) {
    println!("Favorite color: {:?}", color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            name: String::from("John"),
            age: 32,
            color: "blue".to_string(),
        },
        Person {
            name: String::from("MOnjofn"),
            age: 3,
            color: "orange".to_string(),
        },
        Person {
            name: String::from("Smith"),
            age: 10,
            color: "blue".to_string(),
        },
        Person {
            name: String::from("Katty"),
            age: 5,
            color: "pink".to_string(),
        }
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print_name(&person.name);
            println!("Age: {:?}", person.age);
            print_color(&person.color);
        }
    }
}
