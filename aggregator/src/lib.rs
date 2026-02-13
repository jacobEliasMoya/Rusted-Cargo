use uuid::Uuid;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Identify {
    fn identify(&self) -> Uuid;
}

pub trait Loggable {
    fn log_content(&self) -> String;
}

pub struct NewsArticle {
    pub id: Uuid,
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

impl Identify for NewsArticle {
    fn identify(&self) -> Uuid {
        self.id
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

pub fn notify<T: Summary + Identify>(item: &T) {
    println!("Breaking new! {}", item.summarize());
    println!("ID: {}", item.identify());
}

// functionally identical to function above ^^
// just using where to make trait bounds clearer
pub fn notify_where<T>(item: &T)
where
    T: Summary + Identify,
{
    println!("Breaking new! {}", item.summarize());
    println!("ID: {}", item.identify());
}

pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("Concrete username"),
        content: String::from("Concrete content"),
        reply: false,
        repost: false,
    }
}
