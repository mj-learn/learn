fn main() {
    // Simple if
    let num = 40;
    if num < 50 {
        println!("num is lesser than 50");
    }

    let num2 = 65;
    if num2 >= 60 && num <= 70 {
        println!("num2 is between 60 and 70");
    }

    let (f1, f2) = (true, false);
    if f1 == true || f2 == true {
        println!("f1 or f2 is true");
    }

    if f1 != false {
        println!("f1 is not false");
    }

    if (num > 50 && f1 == true) || num2 == 65 {
        println!("one of two conditions is true");
    }

    // if else
    let num3 = 80;
    if num3 < 80 {
        println!("num3 < 80 is true");
    } else {
        println!("num3 < 80 if false")
    }

    // if, else if, else
    let mark = 95;
    let grade: char;
    // or just use let grade;
    if mark >= 90 {
        grade = 'A';
    } else if mark >= 80 {
        grade = 'B';
    } else if mark >= 70 {
        grade = 'C';
    } else if mark >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }
    println!("grade is: {}", grade);
}
