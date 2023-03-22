struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "Drink".to_string(),
            count: 5,
        },
        LineItem {
            name: String::from("Apple"),
            count: 10,
        }
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}
