fn main() {
    let v: Vec<i32> = (0..5).collect();
    println!("v = {:?}", v);

    let sv = &v[0..3];
    println!("sv = {:?}", sv);
}
