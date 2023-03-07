fn main() {
    //comment code

    /* 
    * 
    * Multiline comment
    *
    */

    println!("Hello, world!"); // Print line

    println!("");

    print/* ln */!("Code with comment in mid\n");

    println!("");

    //printls!(10); -> Wrong
    println!("The value is {}", 10);

    println!("My first name is {}, and secound name is {}", "MOn", "Jofn");

    println!("");

    print!("This will be
print in multi line\n");

    println!("");

    println!("This will be
print in multi line too");

    println!("");

    // \n - New line
    // \t - Tab
    // \r - Delete preveous line text

    println!("Doing {} from {} years and i {} it", "programming", 20, "like");
    println!("Doing {2} from {1} years and i {0} it", "like", 20, "programming");
    println!("I'm {name} and love to code in {lang}", name = "MOnjofn", lang = "Rust");
    println!("The sum of 25+5 is {}", 25 + 5);
}
