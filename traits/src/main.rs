mod summary;
use summary::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // We have to provide an implementation for all methods without default implementation!
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // Only implement summarize_author, used by summarize
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Shorter version but with its limitations when having multiple args
fn notify_version_1(item: &impl Summary) {
   println!("Breaking news! {}", item.summarize());
}

fn notify_version_2<T: Summary>(item: &T) {
   println!("Breaking news! {}", item.summarize());
}

fn notify_version_3<T>(item: &T)
   where T: Summary
{
   println!("Breaking news! {}", item.summarize());
}

// Returning a type that implements the summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let news_article = NewsArticle {
        headline: "My awesome headline".to_string(),
        location: "here".to_string(),
        author: "me".to_string(),
        content: "Something".to_string(),
    };

    let tweet = Tweet {
        username: "My cool username".to_string(),
        content: "Something in a tweet".to_string(),
        reply: false,
        retweet: true,
    };

    println!("My news article: {}", news_article.summarize());
    println!("My tweet: {}", tweet.summarize());

    println!("Notification:");
    notify_version_1(&news_article);
    notify_version_2(&tweet);
    notify_version_3(&returns_summarizable());
}
