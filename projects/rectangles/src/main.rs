#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
fn main() {
    
    let rectangle: (u32, u32) = (30, 50);
    println!(
        "Tuple version: area is {} square pixels.",
        area(rectangle)
    );
    
    let scale = 2;
    let rectangle_s = Rectangle {
        width: dbg!(scale * 30),
        height: 50
    };
    println!(
        "Struct version: area is {} square pixels.",
        area_s(&rectangle_s)
    );
    // Can still use rectangle_s because main retains ownsership

    println!("rectangle_s is: {:#?}", rectangle_s);
    dbg!(&rectangle_s);
}

// Tuple version
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Struct version
fn area_s(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}