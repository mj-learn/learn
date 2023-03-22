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

struct Standard {
    price: i32,
}

struct Vip {
    holder_name: String,
    price: i32,
}

struct Backstage {
    holder_name: String,
    price: i32,
}

enum Tickets {
    Backstage(Backstage),
    Standard(Standard),
    Vip(Vip)
}

fn main() {
    let tickets = vec![
        Tickets::Standard(Standard {
            price: 2,
        }),
        Tickets::Vip(Vip {
            holder_name: "MOnjofn".to_owned(),
            price: 5,
        }),
        Tickets::Backstage(Backstage {
            holder_name: "Alex".to_owned(),
            price: 9,
        }),
    ];

    for ticket in tickets {
        match ticket {
            Tickets::Standard(Standard { price }) => println!("Standard ticket for ${:?}", price),
            Tickets::Vip(Vip { holder_name: holder, price }) => {
                println!("VIP ticket holded by {:?} for ${:?}", holder, price)
            }
            Tickets::Backstage(Backstage { holder_name: holder, price }) => {
                println!("Backstage ticket holded by {:?} for ${:?}", holder, price)
            }
        }
    };
}
