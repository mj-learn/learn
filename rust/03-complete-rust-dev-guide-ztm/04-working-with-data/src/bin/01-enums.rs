enum Direction {
    Left,
    Right,
    Up
}

fn main() {
    let go = Direction::Right;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),
    }
}
