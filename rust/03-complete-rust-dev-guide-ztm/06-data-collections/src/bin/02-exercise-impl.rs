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
    Blue,
    Green,
    Pink
}

// * Use a struct to encapsulate the box characteristics
struct BoxDimension {
    width: i32,
    height: i32,
    depth: i32,
}

struct ShippingBox {
    dimensions: BoxDimension,
    weight: i32,
    color: BoxColor,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new_box() -> Self {
        Self { 
            dimensions: BoxDimension {
                width: 3,
                height: 5,
                depth: 8,
            },
            weight: 10,
            color: BoxColor::Blue,
        }
    }

    fn new(dimensions: BoxDimension, weight: i32, color: BoxColor) -> Self {
        Self { dimensions, weight, color }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print_box(&self) {
        match self.color {
            BoxColor::Blue => println!("dimensions w: {}, h: {}, d: {} | weight: {} | color: blue", self.dimensions.width, self.dimensions.height, self.dimensions.depth, self.weight),
            BoxColor::Green => println!("dimensions w: {}, h: {}, d: {} | weight: {} | color: green", self.dimensions.width, self.dimensions.height, self.dimensions.depth, self.weight),
            BoxColor::Pink => println!("dimensions w: {}, h: {}, d: {} | weight: {} | color: pink", self.dimensions.width, self.dimensions.height, self.dimensions.depth, self.weight)
        }
    }
}

fn main() {
    let my_box = ShippingBox::new_box();
    my_box.print_box();

    let my_box_2 = ShippingBox::new(BoxDimension { width: 9, height: 9, depth: 9 }, 8, BoxColor::Pink);
    my_box_2.print_box();

    let my_box_3 = ShippingBox {
        dimensions: BoxDimension {
            width: 2,
            height: 2,
            depth: 2,
        },
        weight: 32,
        color: BoxColor::Green,
    };
    my_box_3.print_box();
}
