// Ownership
// - Each value in Rust has a variable that's called its Ownership
// - There can be only one owner at a time
// - When the owner goes out of scope, the value will be dropped

fn main() {
    // Move
    let _x = vec!["MOnjofn".to_string()]; // on the heap 
    let _y = _x; // give ownership to _y borrow from _x -> (Move)
    
    // Clone expensive operation
    let _x = vec!["MOnjofn".to_string()]; // on the heap 
    let _y = _x.clone();
    let _z = _y.clone();
    println!("_x = {:?}, _y = {:?}, _z = {:?}", _x, _y, _z);
    println!();

    // Copy
    let x = 1; // on the stack
    let y = x; // Copy x
    println!("x = {}, y = {}", x, y);
    println!();

    // More
    let s = String::from("takes"); // on the heap
    takes_ownership(s); // give ownership to the function -> (Move)
    // println!("s = {}", s);
    println!();

    let one = 1; // on the stack
    make_copy(one); // create copy of one
    println!("one = {}", one);
    println!();

    let string = give_ownership(); // string takes ownership of the result
    println!("string = {}", string);
    println!();

    let s2 = "take and give back".to_string();
    println!("s2 = {}", s2);
    let s3 = take_ownership_and_give_back(s2); // Function take ownership from s2 and give it to s3
    // println!("s2 = {}", s2);
    println!("s3 = {}", s3);
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn make_copy(num: i32) {
    println!("num = {}", num);
}

fn give_ownership() -> String {
    "give ownership".to_string()
}

fn take_ownership_and_give_back(string: String) -> String {
    string
}
