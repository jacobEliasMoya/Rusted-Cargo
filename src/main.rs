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

// multiple embedded types within the enum below
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
    }
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

    let m: Message = Message::Write(String::from("Bananananan"));
    m.call();

    println!("{}" );

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
