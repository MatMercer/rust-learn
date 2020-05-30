enum IpKind {
    V4,
    V6,
}

enum IpKindWithValues {
    V4(String),
    V6(String),
}

enum MyIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct MoveMessage { x: i32, y: i32 }
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Message {
    Quit,
    Move(MoveMessage),
    Write(String),
    ChangeColor(ChangeColorMessage),
}

impl Message {
    fn call(&self) {
    }
}

fn main() {
    let kind = IpKind::V4;
    let local = IpKindWithValues::V4(String::from("127.0.0.1"));
    let google_dns = MyIpAddr::V4(8, 8, 8, 8);

    let msg = Message::Write(String::from("Hello there"));
    msg.call();
}

