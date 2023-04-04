fn main() {
    let maybe_user = Some("Jerry");

    match maybe_user {
        Some(user) => println!("user is {:?}", user),
        _ => println!("no user")
    }

    if let Some(user) = maybe_user {
        println!("user is {:?}", user);
    }
}
