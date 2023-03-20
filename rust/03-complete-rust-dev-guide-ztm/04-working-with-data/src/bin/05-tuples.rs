fn main() {
    let coord = (2,3);
    println!("{}, {}", coord.0, coord.1);
    println!("{:?}", coord);
    println!();

    let (x, y) = coord;
    println!("{} {}", x, y);
    println!();

    let (name, age) = ("Monjofn", 18);
    println!("name is {}, age is {}", name, age);
}
