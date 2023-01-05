fn test_dbg() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect);
}

fn calculate_area() {
    let rect = Rectangle {
        width: 30, 
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    println!("rect is {:#?}", rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}