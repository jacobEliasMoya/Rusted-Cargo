// Error Handling in Rust
use std::fs::File;

fn main() {
    // let v: Vec<i32> = vec![1, 2, 3];

    // for i in &v {
    //     println!("{i}");
    // }

    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    println!("{greeting_file:?}");
}
