use uuid::Uuid;
// Defining Shared Behavior with Traits
use aggregator::{NewsArticle, Summary, notify, returns_summarizable};

fn main() {
    let article = NewsArticle {
        id: Uuid::new_v4(),
        headline: String::from("Bananas Poison"),
        location: String::from("California"),
        author: String::from("Ron Burgundy"),
        content: String::from("This just in, bananas are poison"),
    };

    let new_post = returns_summarizable();

    println!("This is the new post {}", new_post.summarize());

    notify(&article);
}
