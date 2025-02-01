pub trait Summary {
    // without a default, this could be written as;
    // fn summarise(&self) -> String;
    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }

    fn summarise_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // Uncomment this, to overwrite the default!
    // fn summarise(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarise_author(&self) -> String {
        format!("#{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
}
