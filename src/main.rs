// Common collections
fn main() {
    let mut v:Vec<i32> = Vec::new();
    // let vec1: Vec<i32> = vec![1,2,3];

    v.push(1);
    v.push(2);
    v.push(3);

    let third: &i32 = &v[3];

    println!("{v:?}");
}
