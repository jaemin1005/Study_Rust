use trait_test::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of cource, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new Tweet: {}", tweet.summarize());
}