#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

impl Message {
    fn print(&self) {
        println!("calling message {:?}", self);
    }
}

fn main() {
    let v4 = IpAddrKind::V4(127, 0, 0, 1);
    let v6 = IpAddrKind::V6(String::from("::1"));

    println!("Ipv4: {:#?}, Ipv6: {:#?}", v4, v6);

    let msg = Message::Quit;
    msg.print();
    let msg = Message::Move{ x: 1, y:2};
    msg.print();
    let msg = Message::Write(String::from("Hello enum"));
    msg.print();
    let msg = Message::ChangeColor(Color(1,2,3));
    msg.print();
}
