// Let's bring our enum back
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        // We can destructure the values from the enum variant
        Message::Move { x, y } => {
            println!("Move in the x direction {} and y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}

fn main() {
    process_message(Message::Write(String::from("Rust is cool")));
    process_message(Message::Move { x: 5, y: -5 });
}