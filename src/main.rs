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
    let mut user1: User = User {
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

    // using dot notation to update struct fields / must have the entire mutable item using the struct marked as mutable
    user1.active = false;
    println!("{}", user1.username);

    user1.username.push_str(" Freaking Rocks");
    println!("{}", user1.username);

    let mut user_mark: User = new_user(
        String::from("Marky Mark"),
        String::from("MarkyMark@gmail.com"),
    );

    println!("New user: {}", user_mark.username);

    user_mark.username.push_str(" and the funky bunch");
    println!("New user adjustment: {}", user_mark.username);

    // new user instance using strcut update syntax
    let user1_alt: User = User {
        active: true,
        username: user1.username,
        email: user1.email,
        sign_in_count: 1,
    };

    println!(
        "User1 instance, cannot use old user1 String as user1_alt now owns it: {}",
        user1_alt.username
    );
}

// function to create user - using shorthand here, func params match struct field name to ensure this works
fn new_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
