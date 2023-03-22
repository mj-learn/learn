#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Worker
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i32,
}

fn print_employee(employee: Employee) {
    println!("{:?}", employee);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    println!("{:?}", me.position);
    println!("{:?}", me);
    println!();

    let you = Employee {
        position: Position::Manager,
        work_hours: 55,
    };
    print_employee(you);
    print_employee(you);
}
