// name fields struct
struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}

// tuple like struct
struct Coordinates(i32, i32, i32);

// unit like struct
struct UnitStruct;

fn main() {
    // Use of name fileds struct
    let u1 = User{active: true, username: "MOnjofn".to_string(), sign_in_count: 0};
    println!("username is {}", u1.username);
    println!("active is {}", u1.active);
    println!("sign_in_count is {}", u1.sign_in_count);
    println!();

    let u2 = build_user(String::from("John Smith"));
    println!("username is {}", u2.username);
    println!("active is {}", u2.active);
    println!("sign_in_count is {}", u2.sign_in_count);
    println!();

    //use of tuple like struct
    let cords = Coordinates(1,2,3);
    println!("cords.0 = {}", cords.0);
    println!("cords.1 = {}", cords.1);
    println!("cords.2 = {}", cords.2);
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}
