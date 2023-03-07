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
