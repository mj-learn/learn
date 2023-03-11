fn main() {
    // loop 
    // loop {
    //     // Infinity loop
    //     println!("Hello, world!");
    // }

    // while loop
    let my_num = 5;
    // let mut guess = false;
    let mut guess = true;

    if guess != true {
        println!("Guess my number between 1 and 10: ");
    }

    while guess != true {
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).expect("Error");
        let number: u8 = number.trim().parse().expect("Error"); 

        if number == my_num {
            guess = true;
            println!("You guessed the number correctly");
        } else {
            println!("Please try again: ");
        }
    }

    // Secound while sample
    let mut num = String::new();
    
    println!("Enter a number and i will tell you the next number after your number that is devisible by both 2 and 5");

    std::io::stdin()
        .read_line(&mut num)
        .expect("Error");

    let mut num: i32 = num.trim().parse().expect("Error");
    num = num + 1;

    while (num % 2 == 0 && num % 5 == 0) != true {
        num += 1;
    }

    println!("The number after your number devisible by both 2 and 5 is {}", num);


}
