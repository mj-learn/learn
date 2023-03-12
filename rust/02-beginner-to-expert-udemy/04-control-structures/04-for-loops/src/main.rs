fn main() {
    // For loops

    for i in 0..5 {
        print!("{} ", i);
    }

    println!("");

    for i in 0..=5 {
        print!("{} ", i);
    }

    println!("");

    let mut vector = vec![41,30,42,25,43,21,44,38,45,85,90,15,11,13,26,33,76,81,69,50];
    for i in &vector { // vector.iter() and vector.iter_mut() are the same thing as & and &mut
        print!("{}, ", i);
    }

    println!();

    for i in &mut vector { // vector.iter() and vector.iter_mut() are the same thing as & and &mut
        *i +=1;
    }
    println!("{:?}", vector);

    println!("\n");

    for i in 0..=5 {
        println!("The {}th element in vector is {}", i, &vector[i]);
    }
}
