// Common collections
fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let vec1: Vec<i32> = vec![1,2,3];

    v.push(1);
    v.push(2);
    v.push(2);
    v.push(2);
    v.push(2);

    // let third: &i32 = &v[2];  will intentionally cause panic -

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("Good to go, valid index here: {third}"),
        None => println!("Error, no valid index"),
    }

    for i in &v {
        println!("{i}");
    }
    // println!("In vector:{v:?} '{third:?}' is the 3rd item");

    for i in &mut v {
        *i += 50;
    }

    println!("{v:?}");
}
