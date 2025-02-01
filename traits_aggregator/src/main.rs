use traits_aggregator::{NewsArticle, Summary, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of couse, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarise());

    let article = NewsArticle {
        headline: String::from("Penguins win the Champions league!"),
        location: String::from("Antarctica"),
        author: String::from("Iceburgh"),
        content: String::from("The penguins win their 27th Champions league cup!"),
    };

    println!("New article available! {}", article.summarise());
}
