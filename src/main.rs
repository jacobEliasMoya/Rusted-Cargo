#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        (2 * self.width) + (2 * self.height)
    }
    fn just_width(&self) -> u32 {
        self.width
    }
    fn just_height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 10,
    };

    println!("The perimeter of rect1 is {}", rect1.perimeter());
    println!("The area of rect1 is {}", rect1.area());
    println!("The width of rect1 is {}", rect1.just_width());
    println!("The height of rect1 is {}", rect1.just_height());
}
