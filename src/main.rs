fn main() {
    // working on Referencing and Borrowing

    let s1: String = String::from("Hello"); // setting s1 to

    let len: usize = calculate_length(&s1); //calling calc_len function referencing s1 

    println!("The length of {s1}: {len}"); // printing 

    // ________________________________________________________//

    let mut s:String = String::from("Hello");

    change(&mut s);

    println!("{s}");
}

fn calculate_length(s: &String) -> usize {
    s.len() //return string length
}

fn change(some_string:&mut String) {
    some_string.push_str(", World");
}
