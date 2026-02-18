// Lifetimes and Rust
// lifetimes = ensuring that references are valid

fn main() {
    // let r;
    //
    // let x: u8 = 5;
    // r = &x;
    //
    // println!("r: {r}");

    let string1 = String::from("abcde");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
