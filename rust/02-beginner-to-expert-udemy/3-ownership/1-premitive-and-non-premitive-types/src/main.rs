// Ownership
// - Each value in Rust has a variable that's called its Ownership
// - There can be only one owner at a time
// - When the owner goes out of scope, the value will be dropped

fn main() {
    let x = 36.6;
    let y = x;
    let yy = &x;
    println!("x = {}, y = {}, yy = {}", x, y, yy);

    let s1 = String::from("abc");
    let s2 = &s1;
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!();

    let v1 = vec![5,6,7,8,9,];
    let v2 = &v1;
    let v3 = v1.clone();
    println!("v1 = {:?}, v2 = {:?}, v3 = {:?}", v1, v2, v3);
}
