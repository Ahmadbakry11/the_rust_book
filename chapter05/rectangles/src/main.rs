#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    // calculate the of the Rectangle using predefined variables.
    // let width: u32 = 50;
    // let height: u32 = 90;

    // println!(
    //     "The area of the rectanglr is {}",
    //     area(width, height)
    // );

    // using Tuples

    // let dimensions = (90, 95);

    // println!(
    //     "The area of the rectanglr is {}",
    //     area_using_tuple(dimensions)
    // );

    // Refactor using Structs
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 900
    };

    println!(
        "The area of the rectangle is {}",
        rectangle_area(&rect1)
    );

    println!("The rectangle is {rect1:#?}");

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_using_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rectangle_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
