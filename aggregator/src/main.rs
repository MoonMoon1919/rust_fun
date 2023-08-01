use aggregator;
use aggregator::{Summary, Tweet, Blog, SmokeSignal};

fn main() {
    let tweet = Tweet {
        username: String::from("horse"),
        content: String::from(
            "of course, as you probably know"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let blog = Blog {
        username: String::from("horse"),
        content: String::from("A blog written by a horse, of course"),
    };

    println!("1 new blog: {}", blog.summarize());
    aggregator::notify(&blog);

    let tweet2 = Tweet {
        username: String::from("cow"),
        content: String::from("MOOOOO"),
        reply: false,
        retweet: false,
    };

    aggregator::double_notify_diff_type(&tweet, &blog);
    aggregator::double_notify_same_type(&tweet, &tweet2);

    let smoke = SmokeSignal {
        creator: String::from("Wilson"),
        content: String::from("Send more volleyballs"),
    };

    aggregator::notify_multi_impl(&smoke);
    aggregator::notify_multi_impl_2(&smoke);
    aggregator::notify_multi_impl_3(&smoke);

    let horse_tweet = aggregator::returns_summarizable(String::from("horse"), String::from("of course"));

    println!("New tweet {}", horse_tweet.summarize())
}
