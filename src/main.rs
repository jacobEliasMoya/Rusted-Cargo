use rand::Rng;

// The match Control Flow Construct
#[derive(Debug)]
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
    let penny: Coin = Coin::Penny;
    let nickel: Coin = Coin::Nickel;
    let dime: Coin = Coin::Dime;
    let quarter: Coin = Coin::Quarter(UsState::California);

    let penny_value: u8 = values_in_cents(&penny);
    let nickel_value: u8 = values_in_cents(&nickel);
    let dime_value: u8 = values_in_cents(&dime);
    let quarter_value: u8 = values_in_cents(&quarter);

    println!(
        "Penny: {penny_value}, Nickel: {nickel_value}, Dime: {dime_value}, Quarter: {quarter_value}"
    );

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    println!("{five:?} {six:?} {none:?}");

    let dice_roll: u8 = rand::thread_rng().gen_range(1..=7);

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {
    println!("Got a fancy hat")
}

fn remove_fancy_hat() {
    println!("Removed fancy hat")
}

// fn reroll() {
//     println!("Rerolling")
// }

fn values_in_cents(coin: &Coin) -> u8 {
    match &coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter is from: {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
