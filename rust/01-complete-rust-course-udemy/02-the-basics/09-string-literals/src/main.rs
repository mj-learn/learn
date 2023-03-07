fn main() {
    let rust = "\x52\x75\x73\x74"; // \x52\x75\x73\x74 = Rust
    let string = "Rust";
    let string2 = "Rust".to_string();
    println!("rust = {}", rust);
    println!("{}", rust == string);
    println!("{}", rust == string2);
}
