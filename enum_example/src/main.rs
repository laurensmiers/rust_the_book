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
    YetAnotherMessage,
}

impl Message {
    fn print(&self) {
        println!("I don't do anything but can be called with any type: {:?}", self);
    }

    fn call(&self) {
        match self {
	    Message::Quit => println!("Calling on a quit type!"),
	    Message::Move { x, y } => println!("Calling on a move type: {}, {}!", x, y),
	    Message::Write (s) => println!("Calling on a write type: {}!", s),
	    Message::ChangeColor (c) => println!("Calling on a color type: {:#?}!", c),
	    _ => println!("Default case in a match"),
	}
    }
}

fn main() {
    let v4 = IpAddrKind::V4(127, 0, 0, 1);
    let v6 = IpAddrKind::V6(String::from("::1"));

    println!("Ipv4: {:#?}, Ipv6: {:#?}", v4, v6);

    let msg = Message::Quit;
    msg.print();
    msg.call();
    let msg = Message::Move{ x: 1, y:2};
    msg.print();
    msg.call();
    let msg = Message::Write(String::from("Hello enum"));
    msg.print();
    msg.call();
    let msg = Message::ChangeColor(Color(1,2,3));
    msg.print();
    msg.call();
    let msg = Message::YetAnotherMessage;
    msg.print();
    msg.call();
}
