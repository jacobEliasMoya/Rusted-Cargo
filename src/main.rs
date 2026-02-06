// Defining Shared Behavior with Traits
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let post = NewsArticle {
        headline: String::from("Bananas Poison"),
        location: String::from("California"),
        author: String::from("Ron Burgundy"),
        content: String::from("This just in, bananas are poison"),
    };
    println!("1 NewsArticle: {}", post.summarize());
}
