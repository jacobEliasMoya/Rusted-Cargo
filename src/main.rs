//The Slice Type
fn main() {
    let string: String = String::from("Find the word dog");
    let f_word: () = find_dog(&string);

    println!("{:?}", f_word);
}

// attempt at byte level pattern searching .)_.) ... learning curve bigtime but its fun
fn find_dog(s: &String) {
    let bytes: &[u8] = s.as_bytes();

    let target: &[u8; 3] = b"dog";

    for (i, &item) in target.iter().enumerate() {
        println!("{item}");
    }
}

