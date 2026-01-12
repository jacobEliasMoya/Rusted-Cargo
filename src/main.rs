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

    let mut s1:String = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    println!("{s1}");


}
