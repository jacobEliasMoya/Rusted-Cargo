//

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number: std::prelude::v1::Option<i32> = None;
    let some_char: std::prelude::v1::Option<char> = Some('e');

    let absent_number: std::prelude::v1::Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_char);
    println!("{:?}", absent_number);

    match some_number {
        Some(n) => println!("{}", n),
        None => println!("Some Number: No Value"),
    }

    match some_char {
        Some(n) => println!("{}", n),
        None => println!("Some Number: No Value"),
    }

    match absent_number {
        Some(n) => println!("{}", n),
        None => println!("Some Number: No Value"),
    }

    let x: i8 = 5;
    let y:std::prelude::v1::Option<i32> = Some(5);

    let sum = x + y;
}
