// concise control flow
#[derive(Debug)]
enum UsState {
    California,
}
enum Coin {
    Dime,
    Quarter(UsState),
}
fn main() {
    // saving numeric literal
    let config_max: Option<u8> = Some(3u8);

    match config_max {
        Some(max) => println!("Max configured to {max}"),
        _ => (),
    };

    if let Some(max) = config_max {
        println!("Max configured to {max}")
    } else {
        println!("Not a max num")
    };

    let mut count: i32 = 0;
    let coin1: Coin = Coin::Quarter(UsState::California);

    // match coin1 {
    //     Coin::Quarter(state) => println!("State Quarter from {state:?}"),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin1 {
        println!("State is {state:?}")
    } else {
        count += 1;
    }

    println!("{count}");
}
