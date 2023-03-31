// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

struct Stock {
    stock: i32,
}

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chairs", Stock { stock: 5 });
    stock.insert("Beds", Stock { stock: 3 });
    stock.insert("Tables", Stock { stock: 2 });
    stock.insert("Couches", Stock { stock: 0 });

    let mut total = 0;

    // * Print the name and number of items in stock for a furniture store
    for (furniture, instock) in stock {
        // * If the number of items is 0, print "out of stock" instead of 0
        if instock.stock == 0 {
            println!("{:?}: out of stock", furniture);
        } else {
            total += instock.stock;
            println!("{:?}: {}", furniture, instock.stock);
        }
    }

    // * Print the total number of items in stock
    println!("Total items in stock: {:?}", total);
}
