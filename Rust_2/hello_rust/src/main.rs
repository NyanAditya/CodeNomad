enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move {x: 10, y: 20};
    let msg2 = Message::Write(String::from("hello"));
    let msg3 = Message::Quit;
}