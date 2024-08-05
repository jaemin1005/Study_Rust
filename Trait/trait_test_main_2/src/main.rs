use trait_test_2::{Summary, Tweet, NewsArticle};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",)
    };

    println!("New article available! {}", article.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        uesrname: String::frim("horse_ebooks");
        content: String::from(
            "of course, ay you probably already know, people",
        ),
        reply: false;
        retweet:false;
    }
}

