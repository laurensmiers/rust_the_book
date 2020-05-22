#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let v4 = IpAddrKind::V4(127, 0, 0, 1);
    let v6 = IpAddrKind::V6(String::from("::1"));

    println!("Ipv4: {:#?}, Ipv6: {:#?}", v4, v6);
}
