// Topic: Option
//
// Requirements:
// * Print out the details of a name's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the name's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_number: Option<i32>,
}

fn main() {
    let monjofn = Student {
        name: "MOnjofn".to_owned(),
        locker_number: None,
    };

    println!("student: {:?}", monjofn.name);

    match monjofn.locker_number {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assignment number")
    }

    let mery = Student {
        name: "Mery".to_owned(),
        locker_number: Some(32)
    };

    println!();

    println!("student: {:?}", mery.name);

    match mery.locker_number {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assignment")
    }
}
