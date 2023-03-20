use std::thread::AccessError;

enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    // admin only access
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("Can access: {}", can_access_file);
}
