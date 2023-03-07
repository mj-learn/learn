fn main() {
    // if, else ...
    let x = 1;

    if x > 10 {
        println!("This is true");
    } else if x == 1 {
        println!("Equal");
    } else {
        println!("This is false");
    }

    println!();

    // Loop
    // loop {
    //     println!("Infinity loop"); // Infinity loop
    // }

    let mut num = 0;
    'conter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;
        loop {
            println!("Decrease: {}", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'conter;
            }
            decrease -= 1; // decrease = decrease -1;
        }
        num += 1; // num = num + 1;
    }

    println!();

    // While
    let mut num = 0;
    while num < 5 {
        println!("Num: {}", num);
        num += 1
    }

    println!();

    // For
    let vec: Vec<i32> = (0..10).collect();
    for vec_element in vec {
        println!("Element is: {}", vec_element);
    }

    println!();

    for number in (1..4).rev() {
        println!("Number is: {}", number);
    }
}
