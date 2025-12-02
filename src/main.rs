fn main() {
    // working on Referencing and Borrowing

    let s1: String = String::from("Hello"); // setting s1 to

    let len: usize = calculate_length(&s1); //calling calc_len function referencing s1 

    println!("The length of {s1}: {len}"); // printing 

    // ________________________________________________________//

    let mut s: String = String::from("Hello");

    change(&mut s);

    println!("{s}");

    // ________________________________________________________//

    {
        let r1: &String = &mut s;
        println!("{r1} is in a different scope");
    }

    let r2: &String = &mut s;
    println!("{r2}");

    let mut mutable_string: String = String::from("value");

    let immutable_one: &String = &mutable_string;
    let immutable_two: &String = &mutable_string;

    println!("These are immutable {immutable_one} & {immutable_two}");

    let mutable_three: &String = &mut mutable_string;
    println!("This is mutable {mutable_three}");

    // ________________________________________________________//

    // let reference_to_do_nothing: &String = dangler();

    let no_dangle_ref: String = no_dangle();

    println!("This is no_dangle_ref {no_dangle_ref}")
}

fn calculate_length(s: &String) -> usize {
    s.len() //return string length
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

// ententionally creating error
// fn dangler() -> &String {
//     let s: String = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s: String = String::from("Hello No Dangle");

    s
}
