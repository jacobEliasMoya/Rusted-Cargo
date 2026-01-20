// Error Handling in Rust

fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

    for i in &v {
        println!("{i}");
    }

    v[99];
}
