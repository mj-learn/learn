fn main() {
    let name = String::from("MOnjofn");
    println!("name = {}", name);

    let name = "Monjofn".to_string();
    println!("name = {}", name);
    
    let new_name = name.replace("Monjofn", "MJ");
    println!("new_name = {}", new_name);

    let str = "MOnjofn"; // String slice - &str
    let string = str.to_string(); // Convert to string
    println!("str = {}", str);
    println!("string = {}", string);
    println!();


    // Compare strings
    println!("{}", "ONE" == "one");
    println!("{}", "ONE".to_lowercase() == "one".to_string());
    println!("{}", "ONE".to_string() == "one");
    println!("{}", "ONE".to_lowercase().to_string() == "one");
}
