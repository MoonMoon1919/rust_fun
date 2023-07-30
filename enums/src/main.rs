enum IpAddress {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Handling a message");
    }
}

fn main() {
    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    let msg = Message::Write(String::from("Hello!"));
    msg.call();

    let some_number = Some(5);

    if some_number.is_none() {
        println!("Oh no, some number is none");
    }

    let some_char = Some('e');

    if some_char.is_some() {
        println!("Found some chars")
    }

    let absent_number: Option<i32> = None;

    if absent_number.is_none() {
        println!("Absent number is none")
    }
}
