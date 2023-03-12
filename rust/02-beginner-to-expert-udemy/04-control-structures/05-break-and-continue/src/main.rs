fn main() {

    println!("Add integer:");
    
    let mut input = String::new();
    
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let mut num:i64 = input.trim().parse().expect("Error");

    loop {
        num -= 1;
        if num % 13 == 0 {
            break; // break - stopping a loop
        }
    }

    println!("The highest number lesser than {} divisible by 13 is {}", input.trim(), num);
    println!();

    let mut num:i64 = input.trim().parse().expect("Error");
    let mut count = 1;
    loop {
        num += 1;
        if num % 5 == 0 && num % 3 == 0 {
            println!("The {}th number after {} devisible by 5 and 3 is {}",count , input.trim(), num);
            count += 1;

            if count == 4 {
                break;
            }
        }

        if num % 2 == 0 {
            continue;
        }
        print!("{}, ", num);
    }
    println!();

    // break with return, can only use with simple loop, can't use in for or while loops!
    let mut num:i64 = input.trim().parse().expect("Error");
    let next_even_num:i64 = loop {
        num += 1;
        if num % 2 == 0 {
            break num;
        }
    };
    println!("Next even number after {} is {}", input.trim(), next_even_num);
}
