struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Intro Into Structs");
    println!("_________________________________________\n");

    // defining user1 with the User struct
    let user1: User = User {
        active: true,
        username: String::from("Jake"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
    };

    // printing out and utilizing dot notation to select Srtuct fields
    println!("This is our first user: {}", user1.username);
    println!("_________________________________________\n");
    println!("Is user1 {} is action: {}", user1.username, user1.active);
    println!("_________________________________________\n");
    println!("User1's email is as follows: {}", user1.email);
    println!("_________________________________________\n");
    println!("User1's has signed in: {} times", user1.sign_in_count);
}
