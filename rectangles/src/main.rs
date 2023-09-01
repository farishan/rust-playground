#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 4,
        height: 5,
    };

    println!("rect is {:?}", rect);

    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
