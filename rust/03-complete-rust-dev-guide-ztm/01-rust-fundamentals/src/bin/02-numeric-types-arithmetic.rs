fn sub(a: i32, b:i32) -> i32 {
    a - b
}

fn main() {
    let sum = 2 + 2; // 4
    let subtract = 10 - 5; // 5
    let devision = 10 / 2; // 5
    let mult = 5 * 5; // 25

    let five = sub(8, 3);

    let rem = 6 % 3; // 0
    let rem2 = 6 % 4; // 2

    println!(" sum is {},\n substract is {},\n devision is {},\n mult is {},\n five is {},\n rem is {},\n rem2 is {}", sum, subtract, devision, mult, five, rem, rem2);
}
