// To panic or not to panic!
use std::net::IpAddr;

fn main() {
    let x: String = String::from("this is an insert");
    println!("Lets see if this formatter works as intended: {x}");

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("{home:?}");
}
