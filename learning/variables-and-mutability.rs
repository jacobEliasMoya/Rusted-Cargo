use std::io;

fn main() {
    let mut hours_input: String = String::new();

    println!("Enter hours to be calculated into seconds:");

    io::stdin()
        .read_line(&mut hours_input)
        .expect("Failed to read line");

    let hours_input: u32 = hours_input
        .trim()
        .parse()
        .expect("Should be a Num or something");

    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_MINUTE: u32 = 60;

    let hours_in_seconds: u32 = MINUTES_IN_HOUR * SECONDS_IN_MINUTE * hours_input;

    println!("Hours In Seconds is {}", hours_in_seconds);

    println!("_______________________________________________________________________");

    let x: i32 = 5;

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("Inner scope x = {}", x);
    }

    println!("Outer Scope of x = {}", x);

    println!("_______________________________________________________________________");

    let spaces: &str = " ";
    let spaces: usize = spaces.len();

    println!("Spaces: {}", spaces);
}
