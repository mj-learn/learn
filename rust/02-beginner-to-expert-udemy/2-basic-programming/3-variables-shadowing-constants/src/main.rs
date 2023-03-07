fn main() {
    // Variables

    let (_var1, _var2) = (250, 2.5);
    let _large_number = 1_000_000;

    let x = 255;

    println!("The value of x = 255 in octal {:o}", x);
    println!("The value of x = 255 in hexdecimal {:x}", x);
    println!("The value of x = 255 in binary {:b}", x);
    println!();

    let n1 = 5;
    let n2 = 2.8;
    let n3 = n1 + n2 as i32;
    let n4 = n1 as f64 + n2;
    println!("The value of n3 is {}", n3);
    println!("The value of n4 is {}", n4);
    println!("");

    // Shadowing

    let _s = 5;
    let _s = 5*5;
    println!("s = {}", _s);
    println!("");

    let mut _p = 5;
    let _p = 5*5; // Replace old p with unmutible p

    let r = 5;
    let mut g = 1;
    {
        let r = 'A';
        g = 55;
        println!("r in code segment is {}", r);
    }
    println!("r = {}", r);
    println!("g = {}", g);
    println!();

    // Constants
    // Require manualy add type!

    const MAX_SALARY:u32 = 100_000;
    println!("MAX_SALARY is {}", MAX_SALARY);
}
