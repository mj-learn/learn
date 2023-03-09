struct MyStruct<'a> {
    text: &'a str
}

fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("{}", r);
    } // x is droped here
    // println!("{}", r);

    // &i32 ->
    // &'a i32 ->
    // &'a mut i32 ->

    // Lifetime in struct
    let string = String::from("asd");
    let y = MyStruct{text: string.as_str()};
    println!("y is: {}", y.text);

    let _xyz: &'static str = "I have a static lifetime";
}

fn _example<'a>(x: &'a str) -> &'a str {
    x
}

fn _example2<'a, 'b>(_x: &'a str, y: &'b str) -> &'b str {
    y
}
