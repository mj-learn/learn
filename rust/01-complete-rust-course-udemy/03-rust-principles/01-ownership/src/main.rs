// Ownership
// - Each value in Rust has a variable that's called its Ownership
// - There can be only one owner at a time
// - When the owner goes out of scope, the value will be dropped

fn main() {

    {
        let n = 1; // on the stack
        println!("n: {}", n);

        let mut s = "Hello".to_string(); // on the heap
        s.push_str(" World");
        println!("s: {}", s);

    }

    // num is dropped
    // str is dropped
}
