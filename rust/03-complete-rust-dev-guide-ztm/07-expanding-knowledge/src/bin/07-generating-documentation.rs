// Documentation comments are with ///
// cargo doc --open -> to make documentation and open it

/// A favorite color.
enum Color {
    Red,
    Blue
}

/// A piece of mail.
struct Mail {
    /// The destination address.
    address: String,
}

/// Adds two numbers together.
fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn main() {}
