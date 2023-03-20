struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    let apple = GroceryItem {
        stock: 5,
        price: 2.89,
    };

    println!("Apples in stock {}", apple.stock);
    println!("Apple price is {}", apple.price);
}
