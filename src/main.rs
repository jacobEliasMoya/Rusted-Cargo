// The match Control Flow Construct

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let penny: Coin = Coin::Penny;
    let nickel: Coin = Coin::Nickel;
    let dime: Coin = Coin::Dime;
    let quarter: Coin = Coin::Quarter;

    let penny_value: u8 = values_in_cents(&penny);
    let nickel_value: u8 = values_in_cents(&nickel);
    let dime_value: u8 = values_in_cents(&dime);
    let quarter_value: u8 = values_in_cents(&quarter);

    println!(
        "Penny: {penny_value}, Nickel: {nickel_value}, Dime: {dime_value}, Quarter: {quarter_value}"
    );
}

fn values_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
