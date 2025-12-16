enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Ch.6: Enums");

    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn route(ip_kind: IpAddrKind) {}
