//The Slice Type
fn main() {
    let string: String = String::from("This is the value that I am looking at");
    first_word(&string);
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
