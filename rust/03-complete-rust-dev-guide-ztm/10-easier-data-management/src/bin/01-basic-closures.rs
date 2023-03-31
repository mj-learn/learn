fn add_fn (a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add_fn(5, 5);
    println!("{:?}", sum);

    let add = |a: i32, b: i32| -> i32 {
        a + b
    };

    let sum = add(2,2);
    println!("{:?}", sum);

    let add = |a, b| a + b;

    let sum = add(1,1);
    println!("{:?}", sum);
}
