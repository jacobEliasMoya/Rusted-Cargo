// concise control flow
enum UsState {
    California,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    // saving numeric literal
    let config_max: Option<u8> = Some(3u8);

    // match config_max {
    //     Some(max) => println!("Max configured to {max}"),
    //     _ => (),
    // }
    
    if let Some(max) = config_max {
        println!("Max configured to {max}")
    } else {
        println!("Not a max num")
    };

    let mut count = 0;
}
