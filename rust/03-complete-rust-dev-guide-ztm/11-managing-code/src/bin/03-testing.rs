fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {
    let word = all_caps("hello world");
    println!("{:?}", word);
}

#[cfg(test)]
mod testt {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("Hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string shoud be all uppercase");
    }
}
