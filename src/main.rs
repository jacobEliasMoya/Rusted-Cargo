use std::collections::HashMap;

fn main() {
    println!("badum hashmap");

    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 58);
    scores.insert(String::from("Yellow"), 158);

    println!("{scores:?}");

    let team_name: String = String::from("Blue");
    let score: u32 = scores.get(&team_name).copied().unwrap_or(0);

    println!("This is the score of {team_name}: {score}");

    for (k, v) in scores {
        println!("This is the score of {k}: {v}");
    }

    let field_name: String = String::from("Favorite color");
    let field_value: String = String::from("Royal Blue");

    let mut new_map: HashMap<String, String> = HashMap::new();

    new_map.insert(field_name, field_value);

    for (k, v) in &new_map {
        println!("My {k} is {v}");
    }

    let updated_field: String = String::from("Hot Pink");

    new_map.insert(String::from("Favorite color"), updated_field);

    for (k, v) in &new_map {
        println!("My {k} is {v}");
    }
}
