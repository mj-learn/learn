// Function ownership and references 

fn main() {
    let num = 1;
    stack(num);
    stack(22);
    println!("num = {}", num);


    let mut vec = vec![4,5,6,7,8,9];
    let vec2 = vec![1,2,3,];
    heap(&mut vec);
    println!("vec = {:?}", vec);

    let mut ref1 = &vec;
    println!("ref1 = {:?}", ref1);
    ref1 = &vec2;
    println!("ref1 = {:?}", ref1);
    println!();

    let large_data1 = String::from("This is first large string.");
    let large_data2 = String::from("This is secound large string.");

    let concatenate = vec![&large_data1, &large_data2];
    println!("concatenate = {:?}", concatenate);
}

fn stack(mut _var:i32) {
    _var = 56;
    println!("_var = {}", _var);
}

fn heap(vec: &mut Vec<i32>){
    vec.push(50);
    println!("vec from fn = {:?}", vec);
}
