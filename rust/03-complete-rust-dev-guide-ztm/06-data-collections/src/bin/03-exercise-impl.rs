// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum BoxColor {
    Silver,
    Pink
}

impl BoxColor {
    fn print(&self) {
        match self {
            Self::Silver => println!("Color: Silver"),
            Self::Pink => println!("Color: Pink")
        }
    }
}

struct Dimension {
    width: f64,
    heigth: f64,
    depth: f64,
}

impl Dimension {
    fn new() -> Self {
        Self {
            width: 3.5,
            heigth: 3.5, 
            depth: 6.5, 
        }
    }

    fn print(&self) {
        println!("Dimension: {}, {}, {}", self.width, self.heigth, self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimension,
    weight: f64,
    color: BoxColor,
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new_box() -> Self {
        Self {
            dimensions: Dimension::new(),
            weight: 9.0,
            color: BoxColor::Silver
        }
    }

    fn print(&self) {
        self.dimensions.print();
        println!("Weight is {}", self.weight);
        self.color.print();
    }
}

fn main() {
    let my_box = ShippingBox::new_box();
    my_box.print();

    println!();

    let my_pink_box = ShippingBox {
        dimensions: Dimension { width: 2.1, heigth: 9.7, depth: 0.2 },
        weight: 55.7,
        color: BoxColor::Pink,
    };
    my_pink_box.print();
}
