struct Temperature {
    degrees_c: f64,
}

impl Temperature {
    fn show_temp(&self) {
        println!("{:?} degrees C", self.degrees_c);
    }

    fn freezing() -> Self {
        Self {
            degrees_c: 5.0,
        }
    }
}

fn main() {
    let hot = Temperature {
        degrees_c: 35.0,
    };
    Temperature::show_temp(&hot);
    hot.show_temp();
    
    println!();

    let cold = Temperature::freezing();
    cold.show_temp();
}
