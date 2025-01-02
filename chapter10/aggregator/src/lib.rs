pub mod tweets;
pub mod news_atricles;
pub mod traits;

pub use traits::Summary;
pub use tweets::Tweet;
pub use news_atricles::NewsArticle;

pub fn collect_tweets() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let notification = notify(&tweet);

    println!("News feed: {notification}");

    println!("The tweet summary is {} and is tweeted by: {}", tweet.summarize(), tweet.summarize_author());
}

pub fn collect_articles() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        published: true,
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let notification = notify(&article);
    
    println!("Breaking news: {notification}");

    println!("The article summary is {}", article.summarize());

    if check_state(&article) == true {
        println!("The article has been published!");
    } else {
        println!("The article has not been published yet!");
    }
}

pub fn notify(item: &impl Summary) -> String {
   format!("Something new: {}", item.summarize()) 
}

pub fn check_state<T: Summary>(item: &T) -> bool {
    item.state()
}
