// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector



fn main() {
    // * Use a vector to store 4 numbers
    let my_vector = vec![10, 20, 30, 40];

    // * Use the .len() function to print the number of elements in a vector
    println!("Vector length is {:?}:", my_vector.len());

    // * Iterate through the vector using a for..in loop
    for item in &my_vector {

        // * Determine whether to print the number or print "thirty" inside the loop
        if item == &30 {
            println!("thirty");
            continue;
        }

        println!("{:?}", item);
    }

    println!();

    let my_numbers = vec![10, 20, 30, 40];

    for num in &my_numbers {
        match num {
            30 => println!("thurty"),
            _ => println!("{:?}", num)
        };
    }
    println!("Number of elements = {:?}", my_numbers.len());
}
