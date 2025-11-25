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

    let s1:String = String::from("Bingo Chingo");
    let s2:String = s1.clone();

    println!("This here is a clone of s1: {s2}");



    println!("s1 Pointer: {:p}", s1.as_ptr());
    println!("s1 Length: {}", s1.len());
    println!("s1 Capacity: {}", s1.capacity());

    println!("this is s1's original value: {s1}");
    
    println!("s2 Pointer: {:p}", s2.as_ptr());
    println!("s2 Length: {}", s2.len());
    println!("s2 Capacity: {}", s2.capacity());

    
}
