use std::fmt::format;

// concise control flow
#[derive(Debug)]
enum UsState {
    California,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::California => year >= 1850,
        }
    }
}

enum Coin {
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

    match coin1 {
        Coin::Quarter(state) => println!("State Quarter from {state:?}"),
    }

    // if let Coin::Quarter(state) = coin1 {
    //     println!("State is {state:?}")
    // } else {
    //     count += 1;
    // }

    println!("{count}");
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
