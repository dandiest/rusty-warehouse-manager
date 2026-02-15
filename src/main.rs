use std::fmt::Debug;
use std::io::{self, stdin};

const WAREHOUSE_NAME: &str = "Stan's Warehouse";
const MAX_CAPACITY: u32 = 500;

#[derive(Debug)]
enum Category {
    Electronics,
    Clothing,
    Other,
}

#[derive(Debug)]
struct Object {
    name: String,
    category: Category,
    price: f32,
    amount: u32,
}

fn main() {
    let mut objects: Vec<Object> = Vec::new();
    let mut amount_input = String::new();

    println!("How much objects do you want to insert in the warehouse?");

    std::io::stdin()
        .read_line(&mut amount_input)
        .expect("Error during reading");

    let amount: usize = amount_input
        .trim()
        .parse()
        .expect("Please, insert a valid number.");

    for i in 0..amount {
        println!("Inserting object number {}", i + 1);

        println!("Please, insert the name of the object.");
        let mut obj_name = String::new();
        std::io::stdin()
            .read_line(&mut obj_name)
            .expect("Please, insert a valid object name.");

        let name = obj_name.trim().to_string();

        println!("Please, insert the price.");
        let mut p_input = String::new();
        io::stdin()
            .read_line(&mut p_input)
            .expect("Error during writing");
        let price: f32 = p_input
            .trim()
            .parse()
            .expect("Please, insert a decimal number.");
        println!("Successfully added {} at {}$", name, price);

        let new_object = Object {
            name: name,
            category: Category::Other,
            price: price,
            amount: 1,
        };
        objects.push(new_object);
        let total_value: f32 = objects.iter().map(|obj| obj.price).sum();
        println!("\nTotal warehouse value: ${:.2}", total_value);
    }
    println!("\n--- FULL INVENTORY ---");
    for obj in &objects {
        println!("{:?}", obj)
    }
}
