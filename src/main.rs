// t puTo panic or not to panic!
use std::net::IpAddr;

fn main() {
    let x: String = String::from("Formatter is working");
    println!("Lets see if this formatter works as intended: {x}");

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("{home:?}");

    loop {

        let guess:i32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_)=>continue,
        };

        if guess < 1 || guess > 100 {
            printlin!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number);
    }
}

