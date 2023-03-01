fn main() {
    println!("let tuples = (500, \"string\", true);");

    let tuples = (500, "string", true);
    println!("tuples.0 is {}", tuples.0);

    let (x, y, z) = tuples;
    println!();
    println!("let (x, y, z) = tuples;");
    println!("x, y, z are {}, {}, {}", x, y, z);
}
