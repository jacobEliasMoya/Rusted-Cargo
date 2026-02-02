
use std::io;

#[derive(Debug)]
struct HealthyUser {
    name: String,
    favorite_food: String,
    calories_per_day: u32,
}

fn main() {
    let mut name: String = String::new();
    let mut favorite_food: String = String::new();
    let mut calories_per_day: String = String::new();

    println!("Please enter your name");

    io::stdin()
        .read_line(&mut name)
        .expect("Error reading name");

    println!("Please enter your favorite food.");

    io::stdin()
        .read_line(&mut favorite_food)
        .expect("Could not read favorite food");

    println!("Please enter you daily calorical intake.");

    io::stdin()
        .read_line(&mut calories_per_day)
        .expect("Could not read favorite food");

    let calorie_value: u32 = calories_per_day.trim().parse().expect("Parsing failed");

    let new_user: HealthyUser = HealthyUser {
        name: name,
        favorite_food: favorite_food,
        calories_per_day: calorie_value,
    };

    println!("User: {new_user:?}");
}
