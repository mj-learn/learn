fn main() {

    // Mutable and Immutable References
    //    - One mutable reference for a variable in a scope
    //    - Many immutable references
    //    - Mutable and immutable can not coexist
    //    - Data should not change when immutable references are in scope
    //    - Scope of a reference

    let mut s = String::from("Hello");
    println!("s = {}", s);
    change_string(&mut s);
    println!("s = {}", s);
}

fn change_string(string: &mut String) {
    string.push_str(" world");
}
