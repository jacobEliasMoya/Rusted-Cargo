use std::usize;

fn main() {
    // intro to ownership .. dun dun dun

    let s: &str = "hello";
    println!("This is the string: {s}");

    let mut s_type = String::from("Baba");
    s_type.push_str("-");
    s_type.push_str("yaga");

    println!("Pointer: {:p}", s_type.as_ptr());
    println!("Length: {}", s_type.len());
    println!("Capacity: {}", s_type.capacity());

    let x = 5;
    let y: i32 = x;

    println!("{y}");

    let mut s1: String = String::from("hello");
    s1 = String::from("ahoy");

    println!("{s1} world");

    let s1: String = String::from("Bingo Chingo");
    let s2: String = s1.clone();

    println!("This here is a clone of s1: {s2}");

    println!("s1 Pointer: {:p}", s1.as_ptr());
    println!("s1 Length: {}", s1.len());
    println!("s1 Capacity: {}", s1.capacity());

    println!("this is s1's original value: {s1}");

    println!("s2 Pointer: {:p}", s2.as_ptr());
    println!("s2 Length: {}", s2.len());
    println!("s2 Capacity: {}", s2.capacity());

    let new_string: String = String::from("Hello");

    takes_ownership(new_string);

    let to_be_copied: i32 = 13;
    makes_copy(to_be_copied);

    let s3: String = gives_ownership();


    let s4: String = String::from("hello");
    let s5: String = takes_and_gives_back(s4);

    println!("{s3} {s5}");

    let s6:String = String::from("hello");

    let (s6, len) = calculate_length(s6);

    println!("The length of '{s6}' is {len}.");

}

fn calculate_length(s:String) -> (String,usize) {
    let length:usize = s.len();
    (s,length)
}

fn gives_ownership() -> String {
    let some_string: String = String::from("Yours");
    some_string //returning this here
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string //returning string back
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
