// Defining Shared Behavior with Traits
use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let post = NewsArticle {
        headline: String::from("Bananas Poison"),
        location: String::from("California"),
        author: String::from("Ron Burgundy"),
        content: String::from("This just in, bananas are poison"),
    };

    println!("1 NewsArticle: {}", post.summarize());
}
