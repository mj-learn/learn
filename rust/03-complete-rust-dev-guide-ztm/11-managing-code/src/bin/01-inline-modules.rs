mod greet {
    pub fn hello() {
        println!("hello");
    }

    pub fn goodbye() {
        println!("good bye");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    greet::hello();

    use greet::{hello, goodbye};
    hello();
    goodbye();

    use math::*;
    println!("{:?}", add(2, 2));
    println!("{:?}", sub(5, 3));
}
