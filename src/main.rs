fn main() {
    // let width1: u32 = 30;
    // let height1: u32 = 50;

    // println!(
    //     "The are of the rectangle is {} square pixles.",
    //     area(width1, height1)
    // );

    let rect1: (u32, u32) = (30, 50);

    println!("The are of the rectangle is {} square pixles.", area(rect1));
}

// v1
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
