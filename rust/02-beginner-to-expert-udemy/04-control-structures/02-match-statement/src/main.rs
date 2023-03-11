fn main() {
    // Match statement
    let num = 200;
    match num {
        1 => println!("num is 1"),
        2 | 3 => println!("num is 2 or 3"),
        4..=99 => println!("num is between 4 and 99"),
        100 => {
            let b = true;
            if b {
                println!("num is 100") // have not semicolon becouse need to return this
                                       // same like in functions return
            }
        },
        _ => println!("num is something else"),
    }

    let marks = 50;
    let grade: char;
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    };
    println!("grade is: {}", grade);

    // let match
    let marks2 = 94;
    let grade2 = match marks2 {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("grade2 is: {}", grade2);
}
