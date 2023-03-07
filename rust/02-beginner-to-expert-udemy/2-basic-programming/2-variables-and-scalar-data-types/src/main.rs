fn main() {

    let _a = 32; // i32 Default
    let _b = 1.50; // f64 Default

    let x:f32 = 1.50;
    println!("x is {}", x);

    // Rules of naming variables
    // - Only letters, digits and underscores
    // - Shoud begin with letter or underscore
    // - Case sensitive
    
    let z = 10;
    let m = 5*5;
    println!("z is {} and m is {}", z, m);

    // Data Types
    // Integer - i8, i16, i32, i64 and u8, u16, u32, u64

    println!();
    println!("Max value of i8 is {}", std::i8::MAX);
    println!("Max value of i16 is {}", std::i16::MAX);
    println!("Max value of i32 is {}", std::i32::MAX);
    println!("Max value of i64 is {}", std::i64::MAX);

    println!();
    println!("Min value of i8 is {}", std::i8::MIN);
    println!("Min value of i16 is {}", std::i16::MIN);
    println!("Min value of i32 is {}", std::i32::MIN);
    println!("Min value of i64 is {}", std::i64::MIN);

    println!();
    println!("Max value of u8 is {}", std::u8::MAX);
    println!("Max value of u16 is {}", std::u16::MAX);
    println!("Max value of u32 is {}", std::u32::MAX);
    println!("Max value of u64 is {}", std::u64::MAX);

    println!();
    println!("Min value of u8 is {}", std::u8::MIN);
    println!("Min value of u16 is {}", std::u16::MIN);
    println!("Min value of u32 is {}", std::u32::MIN);
    println!("Min value of u64 is {}", std::u64::MIN);

    // Float - f32, f64
    println!();
    println!("Max value of f32 is {}", std::f32::MAX);
    println!("Max value of f64 is {}", std::f64::MAX);

    println!();
    println!("Min value of f32 is {}", std::f32::MIN);
    println!("Min value of f64 is {}", std::f64::MIN);

    // Bool - bool (true or false)
    // !=, ==, <, >, <=, >=

    let _status = false;
    let status = 18 != 18;
    println!("The value of multiple variables is {:?}", (z,m,status));

    println!();
    // Character - char (always in '..' )

    let c1 = '2';
    let c2 = '+';
    let c3 = 'a';
    let c4 = '\u{283A}';
    let c5 = '\"';
    println!("Values of c1 is {}, c2 is {}, c3 is {}, c4 is {}, c5 is {}", c1, c2, c3, c4, c5);

}
