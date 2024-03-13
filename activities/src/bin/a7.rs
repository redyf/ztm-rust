// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Color {
    Red,
    Blue,
    Yellow,
    Purple,
    Black,
    White,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(my_color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match my_color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Yellow => println!("Yellow"),
        Color::Purple => println!("Purple"),
        Color::Black => println!("Black"),
        Color::White => println!("White"),
    }
}
fn main() {
    print_color(Color::Red);
    print_color(Color::Blue);
    print_color(Color::Yellow);
    print_color(Color::Purple);
    print_color(Color::Black);
    print_color(Color::White);
}
