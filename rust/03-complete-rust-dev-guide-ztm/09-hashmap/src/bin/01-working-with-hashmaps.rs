use std::collections::HashMap;

#[derive(Debug)]
struct Content {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Content { content: "stuff".to_owned() });
    lockers.insert(2, Content { content: "shirts".to_owned() });
    lockers.insert(3, Content { content: "gym shorts".to_owned() });

    for (locker, content) in lockers.iter() {
        println!("locker: {:?}, content: {:?}", locker, content.content);
    }
}
