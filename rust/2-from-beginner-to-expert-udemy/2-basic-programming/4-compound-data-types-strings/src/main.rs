fn main() {
    // &str - fixed size not mutable
    let some_string = "Fixed lenght string"; //&str is default
    println!("some_string is \"{}\"", some_string);
    println!();
    
    // String - can grow
    let mut growable_string = String::from("This string will grow");
    println!("growable_string is \"{}\"", growable_string);

    growable_string.push('s');
    println!("growable_string after push \"{}\"", growable_string);

    growable_string.pop();
    growable_string.pop();
    println!("growable_string after pop 2 times \"{}\"", growable_string);

    growable_string.push_str(" + some text");
    println!("growable_string after push_str \"{}\"", growable_string);
    println!();

    // Common operation
    println!("Basics function on strings:
is_empty(): {},
len(): {},
bytes(): {},
Contains \"use\": {},
Contains \"will\": {}",
    growable_string.is_empty(),
    growable_string.len(),
    growable_string.capacity(),
    growable_string.contains("use"),
    growable_string.contains("will"));

    let num = 6;
    let num_string = num.to_string();
    println!("is num_string string? {}", num_string == "6");
    println!();

    let my_name = "MOnjofn".to_string();
    println!("my_name is {}", my_name);

    let empty_string = String::new(); //Create empty string
    println!("Length of empty_string is {}", empty_string.len());


    // Format strings
    let s_1 = "MOn".to_string();
    let s_2 = "jofn".to_string();
    let s_3 = format!("My name is {}{}", s_1, s_2);
    println!("{}", s_3);
}
