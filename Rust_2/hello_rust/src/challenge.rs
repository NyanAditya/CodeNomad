struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rec1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect_area = area(&rec1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_area
    );
}