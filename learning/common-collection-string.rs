// More into the UTF-8 String

fn main() {
    let s = String::from("initial contents");
    // let mut s: String = String::new();
    // let data: String = "initial contents".to_string();
    // let s: String = data.to_string();
    println!("{s}");

    // updating a string

    let mut str: String = String::from("foo");
    str.push_str("bar");

    println!("{str}");

    let s1: String = String::from("foo");
    let s2 = "bar";
    let s3 = s1 + &s2;

    // s1.push_str(s2);

    // s1.push('l')
    println!("{s3}");

    // string slicing
    let slice = "HelloThereString";
    let selected_slice = &slice[0..11];

    println!("This is the selected_slice {selected_slice}");

    for c in selected_slice.chars() {
        println!("The char is: {c}");
    }


}
