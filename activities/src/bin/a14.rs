// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct Person {
    // * The color and name should be stored as a String
    name: String,
    favorite_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data)
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            name: String::from("Mateus"),
            favorite_color: String::from("blue"),
            age: 20,
        },
        Person {
            name: String::from("Fulano"),
            favorite_color: String::from("blue"),
            age: 9,
        },
        Person {
            name: String::from("Filipe"),
            favorite_color: String::from("red"),
            age: 21,
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print(&person.name);
            print(&person.favorite_color);
            // * The name and colors should be printed using a function
        }
    }
}
