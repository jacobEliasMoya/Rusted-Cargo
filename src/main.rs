#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1: u32 = 30;
    // let height1: u32 = 50;

    // println!(
    //     "The are of the rectangle is {} square pixles.",
    //     area(width1, height1)
    // );

    // let rect1: (u32, u32) = (30, 50);
    // println!("The are of the rectangle is {} square pixles.", area(rect1));

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "The are of the rectangle is {} square pixles.",
        area(&rect1)
    );
}

// v1
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
