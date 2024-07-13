// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    // * The locker assignment should use an Option<i32>
    locker: Option<i32>,
}

fn main() {
    let mateus = Student {
        name: "Mateus".to_owned(),
        locker: Some(3),
    };

    println!("Student: {:?}", mateus.name);
    match mateus.locker {
        Some(num) => println!("You do have a registered locker in slot: {:?}", num),
        None => println!("You don't have a locker assigned"),
    }
}
