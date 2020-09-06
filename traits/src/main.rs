pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

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
        format!("{}", self.username)
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
}
