fn main() {
    println!(" ____________________________________________\n");
    println!("|          Chapter 3.2: Data Types           |");
    println!(" ____________________________________________\n");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("|* This is the guess, parsed as a number {}  *|\n", guess);

    println!(" ____________________________________________\n");

    let negative_number: i8 = -32;
    let positive_number: u8 = 32;

    // so unsigned integers are positive, and unsigned(i) - have a negative
    // was curious why this was popping up, nice little tidbit of info

    println!("This is a signed integer: {}", negative_number);
    println!("This is a unsigned integer: {} \n", positive_number);

    println!("Testing max unsigned num {}", u128::MAX);
    println!("Testing min unsigned num {} \n", u128::MIN);

    println!("Testing max signed num {}", i64::MAX);
    println!("Testing min signed num {}\n", i64::MIN);

    let decimal: u32 = 98_222;
    println!("This should be a decimal: {}", decimal);

    // all floating points are signed
    let floating_point: f32 = -45.87;
    let floating_point_two: f64 = -45.87;
    println!("This is a floating point - f32: {}", floating_point);
    println!("This is a floating point: - f64 {}", floating_point_two);
}
