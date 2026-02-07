// Defining Shared Behavior with Traits
use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("Bananas Poison"),
        location: String::from("California"),
        author: String::from("Ron Burgundy"),
        content: String::from("This just in, bananas are poison"),
    };

    println!("1 NewsArticle: {}", article.summarize());

    let post = SocialPost {
        username: String::from("JoeShmo"),
        content: String::from("There is is a banana"),
        reply: false,
        repost: false,
    };

    println!("New Post: {}", post.summarize());
}
