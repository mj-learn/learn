struct Square {
    width: u32,
    height: u32
}

impl Square {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn what_is_my_width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let sq = Square {width: 5, height: 5};
    println!("{} {}", sq.width, sq.height);
    println!("sq are is: {}", sq.area());
    println!("sq what_is_my_width is: {}", sq.what_is_my_width());

    let mut sq2 = Square {width: 10, height: 10};
    println!("width is: {}", sq2.width);
    sq2.change_width(5);
    println!("width is: {}", sq2.width);
}
