// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Ch.6: Enums");

    // let four: IpAddr = IpAddr::V4(127,0,0,1);
    // let six: IpAddr = IpAddr::V6(String::from("::1"));

    let home: IpAddr = IpAddr::V4(127, 0, 0, 1);
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    println!("This is the home IP: {:?}", home);
    println!("This is the loopback IP: {:?}", loopback);

    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 {}", addr);
        }
    }
    // let home: IpAddr = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback: IpAddr = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
}

// fn route(ip_kind: IpAddrKind) {}
