use std::collections::HashMap;

fn main() {
    println!("badum hashmap");

    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 58);
    scores.insert(String::from("Yellow"), 158);

    println!("{scores:?}");
}
