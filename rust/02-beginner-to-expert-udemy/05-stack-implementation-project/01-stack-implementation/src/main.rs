// Stack implementation

fn new_stack(maxsize: usize)-> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let value = stack.pop();
    println!("The poped value is {:?}", value);
    value
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can not add more");
    } else {
        stack.push(item);
        println!("Stack {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn user_input () -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error");
    let res: u32 = input.trim().parse().expect("Error");
    res
}

fn main() {
    println!("Let us first create a stack for our use");
    println!("Please mention the size of the stack");

    let stack_size: u32 = user_input();
    let mut stack: Vec<u32> = new_stack(stack_size as usize);

    loop {
        println!("\n\n ********* MENU ********* \n");
        println!(" 1) Push\n 2) Pop\n 3) Display\n 4) Size\n 5) Exit");
        println!("Enter your choice number:");
        let choice = user_input();
        match choice {
            1 => {
                println!("Enter value to be insert:");
                let item = user_input();
                push(&mut stack, item, stack_size as usize);
            },
            2 => println!("The element which is poped is {:?}", pop(&mut stack)),
            3 => println!("The elements are: {:?}", stack),
            4 => println!("The size of stack is: {}", size(&stack)),
            _ => {
                println!("Do you want to exit 1 = Yes / 0 = No");
                let status = user_input();
                if status == 1 {
                    break;
                }
            }
        }
    }
}
