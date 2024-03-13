// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity and id number
    id: i32,
    quantity: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn grocery_item_id(item: &GroceryItem) {
    println!("Id: {:?}", item.id);
}

fn grocery_item_quantity(item: &GroceryItem) {
    println!("Quantity: {:?}", item.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn main() {
    let my_item = GroceryItem {
        id: 150,
        quantity: 10,
    };
    grocery_item_id(&my_item);
    grocery_item_quantity(&my_item);
}
