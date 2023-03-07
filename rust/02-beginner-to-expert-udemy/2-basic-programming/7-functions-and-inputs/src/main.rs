// Functions and Inputs

fn main() {
    basic_fn();
    fn_with_inputs("MOnjofn", 10_000);

    let name_input = "MOn Jofn";
    let salary_input = 50_000;
    fn_with_inputs(name_input, salary_input);
    println!();

    let answer = fn_with_output(5, 5);
    println!("{}", answer);
    println!();

    let (multiplication, sum, modul ) = fn_with_multiple_outputs(5, 10);
    println!("multiplication = {}, sum = {}, modul = {}", multiplication, sum, modul);
    println!();

    let result_tuple = fn_with_multiple_outputs(5, 10);
    println!("multiplication = {}, sum = {}, modul = {}", result_tuple.0, result_tuple.1, result_tuple.2);
    println!();

    // Code block
    let full_name = {
        let first_name = "MOn";
        let secound_name = "Jofn";
        format!("{} {}", first_name, secound_name)
    };
    println!("My full name is {}", full_name);

    // Input from user
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error msg");

    let input: f64 = input.trim().parse().expect("Error while parse");
    println!("input is {}", input);
}

fn basic_fn() {
    println!("This is from basic function");
}

fn fn_with_inputs(name: &str, salary: i32) {
   println!("The name is {} and salary is {}", name, salary); 
}

fn fn_with_output(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn fn_with_multiple_outputs(num1: i32, num2:i32) -> (i32, i32, i32) {
    (num1*num2, num1 + num2, num1 % num2)
}
