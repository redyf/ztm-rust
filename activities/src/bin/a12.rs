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
    Red,
    Black,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Red => println!("Red"),
            BoxColor::Black => println!("Black"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("weight: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}
// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    color: BoxColor,
    weight: f64,
    dimensions: Dimensions,
}
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new(color: BoxColor, weight: f64, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let big_dimensions = Dimensions {
        width: 10.0,
        height: 20.0,
        depth: 30.0,
    };
    let small_box = ShippingBox::new(BoxColor::Red, 5.0, small_dimensions);
    let big_box = ShippingBox::new(BoxColor::Black, 50.0, big_dimensions);
    small_box.print();
    big_box.print();
}
