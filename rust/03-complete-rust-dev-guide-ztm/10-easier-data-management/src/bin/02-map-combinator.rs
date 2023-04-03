fn maybe_num() -> Option<i32> {
    Some(1)
}

fn maybe_word() -> Option<String> {
    Some("Word".to_owned())
}

fn main() {
    let number = match maybe_num() {
        Some(num) => Some(num + 1),
        None => None
    };

    println!("{:?}", number);

    let number = maybe_num().map(|num| num + 1);

    println!("{:?}", number);

    let word = maybe_word()
        .map(|w| w.len())
        .map(|len| len * 2);

    println!("{:?}", word);
}
