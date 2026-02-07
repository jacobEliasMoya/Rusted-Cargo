//The Slice Type
fn main() {
    let string: String = String::from("Hello World");
    let first_word: &str = first_word(&string);

    println!("{}", first_word);
    // println!("{}", string);

    // string.clear();

    // println!("{}", first_word);
    // println!("{}", string);

    let first: &str = &string[0..5];
    let last: &str = &string[6..=10];

    println!("{first}_{last}");

    let slice: &str = &string[0..2];
    println!("{slice}");

    let slice: &str = &string[..2];
    println!("{slice}");

    let string_length: usize = string.len();

    let slice: &str = &string[3..string_length];
    println!("{slice}");

    let slice: &str = &string[3..];
    println!("{slice}");

    let slice: &str = &string[0..string_length];
    println!("{slice}");

    let slice: &str = &string[..];
    println!("{slice}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// attempt at byte level pattern searching .)_.) ... learning curve bigtime but its fun
// fn find_dog(s: &String) {
//     let bytes: &[u8] = s.as_bytes();
//     let target: &[u8; 3] = b"dog";
//     for (i, &item) in target.iter().enumerate() {
//         println!("{item}");
//     }
// }

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
