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

// * Use an enum to create different flavors of drinks
enum Flavor {
    Cola,
    Soda,
    Lemonade,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    ounces: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Cola => println!("Flavor is Cola"),
        Flavor::Soda => println!("Flavor is Soda"),
        Flavor::Lemonade => println!("Flavor is Lemonade"),
    };
    println!("Fluid ounces are {:?}", drink.ounces);
}

fn main() {
    let coca_cola = Drink {
        flavor: Flavor::Cola,
        ounces: 7.0,
    };
    print_drink(coca_cola);
    println!();

    let soda = Drink {
        flavor: Flavor::Soda,
        ounces: 5.0,
    };
    print_drink(soda);
    println!();

    let lemonade = Drink {
        flavor: Flavor::Lemonade,
        ounces: 2.0,
    };
    print_drink(lemonade);
    println!();
}
