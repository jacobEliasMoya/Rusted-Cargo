//The Slice Type
fn main() {
    let string: String = String::from("Hello World");
    // let first_word: usize = first_word(&string);

    // println!("{}", first_word);
    // println!("{}", string);

    // string.clear();

    // println!("{}", first_word);
    // println!("{}", string);

    let first: &str = &string[0..5];
    let last: &str = &string[6..=10];

    println!("{first}_{last}");

 }

// attempt at byte level pattern searching .)_.) ... learning curve bigtime but its fun
// fn find_dog(s: &String) {
//     let bytes: &[u8] = s.as_bytes();
//     let target: &[u8; 3] = b"dog";
//     for (i, &item) in target.iter().enumerate() {
//         println!("{item}");
//     }
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }
