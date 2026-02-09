pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Loggable {
    fn log_content(&self) -> String;
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

impl Loggable for NewsArticle {
    fn log_content(&self) -> String {
        format!("{}", self.content)
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

pub fn notify(item: &impl Summary) {
    println!("Breaking new! {}", item.summarize());
}

pub fn notify_both<T: Summary>(item: &T) {
    println!("Breaking new! {}", item.summarize());
}

pub fn log_line(item: &impl Loggable) {
    println!("Line logged: {}", item.log_content());
}
