fn main() {
    let x: i8 = 10;
    println!("x is {}",x);

    let y: u8 = 10;
    println!("y is {}", y);

    let decimal = 02_55;
    println!("decimal is {}", decimal);

    let hex = 0xff;
    println!("hex is {}", hex);

    let octal = 0o337;
    println!("octal is {}", octal);

    let binary = 0b1111_1111;
    println!("binary is {}", binary);

    let byte = b'A';
    println!("byte is {}", byte);

    println!();

    let float = 2.5; //f64 default
    let float2:f32 = 1.5;
    println!("float is {}",float);
    println!("float2 is {}",float2);

    println!();

    let a = 10;
    let b = 4;
    //+ - * / %
    let remainder = a % b;
    println!("10 % 4 = {}",remainder);
}
