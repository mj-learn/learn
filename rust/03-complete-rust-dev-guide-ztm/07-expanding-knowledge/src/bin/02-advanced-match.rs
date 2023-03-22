enum Discount {
    Precent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        2 => println!("2"),
        other => println!("other: {:?}", other),
    };

    let flat = Discount::Flat(5);
    match flat {
        Discount::Flat(2) => println!("flat: 2"),
        Discount::Flat(value) => println!("flat: {:?}", value),
        _ => (),
    };

    let concerts = vec![
        Ticket {
            event: "Kiss".to_owned(),
            price: 50,
        },
        Ticket {
            event: "ACDC".to_owned(),
            price: 20,
        },
    ];

    for ticket in concerts {
        match ticket {
            Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
            Ticket { price, .. } => println!("price = {:?}", price),
        }
    }
}
