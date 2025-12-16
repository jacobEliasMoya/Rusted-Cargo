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
    fn width_bool(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 10,
    };

    let rect2: Rectangle = Rectangle {
        width: 100,
        height: 5,
    };

    println!("The perimeter of rect1 is {}", rect1.perimeter());
    println!("The area of rect1 is {}", rect1.area());
    println!("The width of rect1 is {}", rect1.just_width());
    println!("The height of rect1 is {}", rect1.just_height());

    println!("The rect width is greater than 0: {}", rect1.width_bool());

    println!(
        "The rect1 width can contain rect2: {}",
        rect1.can_hold(&rect2)
    );

    let r: Rectangle = Rectangle::square(55);
    println!("The area of the new r is {}", r.area());
    // println!("The square method on rectangle is: {:?}", r);
}
