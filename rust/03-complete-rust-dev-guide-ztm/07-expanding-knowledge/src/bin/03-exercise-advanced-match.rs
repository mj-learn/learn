// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
enum Tickets {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String)
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Tickets::Standard(2.99),
        Tickets::Vip(5.99, "John Smith".to_owned()),
        Tickets::Backstage(7.99, "Katty".to_owned())
    ];

    for ticket in tickets {
        // * Use a match expression while iterating the vector to print the ticket info
        match ticket {
            Tickets::Backstage(price, name) => { 
                println!("Backstage ticket holded by {:?} for ${:?}", name, price)
            }
            Tickets::Standard(price) => println!("Standart ticket for ${:?}", price),
            Tickets::Vip(price, name) => {
            println!("VIP ticket holded by {:?} for ${:?}", name, price)
            }
        }
    };
}
