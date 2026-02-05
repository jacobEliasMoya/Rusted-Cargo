// Generic Data Types
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }
//

fn main() {
    let number_list = vec![34, 50, 36, 100, 65];
    let result = largest_generic(&number_list);
    println!("This is the largest number: {result}");

    let p = Point { x: 6, y: 7 };

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p_float = Point { x: 1.5, y: 2.4 };

    println!("distance: {}", p_float.distance_from_origin());

    // let char_list = vec!['y', 'm', 'a', 'q', 'r'];
    // let result = largest_generic(&char_list);
    // println!("This is the largest char: {result}");
    //
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 5.0, y: 10.0 };
    // let float_int = Point { x: 20, y: 10.0 };
    //
    // println!("{integer:?}");
    // println!("{float:?}");
    // println!("{float_int:?}");
}
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
//
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
