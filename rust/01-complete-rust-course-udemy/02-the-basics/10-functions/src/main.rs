fn main() {
    print_phrase();
    print_phrase_2("Print my argument");
    println!("{}", gcd(20, 5));
    println!("{}", multiple_return_values(true));
}

fn print_phrase() {
    println!("Hello from the print_phrase function.")
}

fn print_phrase_2(phrase: &str) {
    println!("{}", phrase);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b // return this becouse ; missing
}

fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true // return this
    } else {
        false // or return this
    }
}
