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

    let remainder: u8 = 43 % 5;
    let truncated: i8 = -5 / 3;

    println!("remainder using modulo: {remainder} ");
    println!("truncation: {truncated}");

    let sum: u8 = 5 + 5;
    let difference: u8 = 5 - 4;

    println!("sum is: {sum}");
    println!("difference is: {difference}");

    let product: u8 = 8 * 8;
    let quotient: f64 = 8.2 / 8.0;

    println!("product is: {product}");
    println!("quotient is: {quotient}");

    let true_bool: bool = true;
    let false_bool: bool = false;

    println!("This bool is: {true_bool}");
    println!("This bool is: {false_bool}");

    let character: char = 'B';

    println!("This is a char, remeber just single quote, not double: '{character}'");

    let tup: (i32, f64, u8) = (-456, 4.5, 5);

    let (x, y, z) = tup;

    println!("This is a tuple: {x}, {y}, {z}");

    let four_fifty_six: i32 = tup.0;
    let four_point_five: f64 = tup.1;
    let five: u8 = tup.2;

    println!("Parts of the tuple, indexed accordingly {four_fifty_six}, {four_point_five}, {five}");
}
