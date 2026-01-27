// Error Handling in Rust
use std::fs::File;
// use std::io::ErrorKind;

fn main() {
    // let v: Vec<i32> = vec![1, 2, 3];
    // for i in &v {
    //     println!("{i}");
    // }
    // let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");
    // let greeting_file: File = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // let greeting_file: File = File::open("hello.txt").unwrap();

    let greeting_file: File = File::open("helo.txt").expect("This is an error, no file found"); // expect is a more commonplace appraoch here for err handling


}

fn read_username_from_file()->Result<String,io::Error>{

   let username_file_result = File::open("Hello.txt");
   let mut username_file = match username_file_result {
        Ok(file) => file, 
        Err(e) => return Err(e),
   };

   let mut username = String::new();

   match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
   }

}

