use uuid::Uuid;
// Defining Shared Behavior with Traits
use aggregator::{NewsArticle, notify};

fn main() {
    let article = NewsArticle {
        id: Uuid::new_v4(),
        headline: String::from("Bananas Poison"),
        location: String::from("California"),
        author: String::from("Ron Burgundy"),
        content: String::from("This just in, bananas are poison"),
    };

    notify(&article);
}
