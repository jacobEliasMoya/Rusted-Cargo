// Lifetimes and Rust
// lifetimes = ensuring that references are valid
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // let r;
    // let x: u8 = 5;
    // r = &x;
    // println!("r: {r}");

    let string1 = String::from("abcde");
    let string2 = "xyz";
    let int1 = 123;
    let int2 = 1234234;

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let result2 = greatest_int(&int1, &int2);
    println!("The longest string is {result2}");

    let novel = String::from("This is a novel, it has letters...");
    let first_part = novel.split(',').next().unwrap();
    let i = ImportantExcerpt { part: &first_part };
    println!("{}", i.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn greatest_int<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y { x } else { y }
}
