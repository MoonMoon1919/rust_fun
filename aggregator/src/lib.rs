use std::fmt;

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
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Uses the default implementation of summarize
pub struct Blog {
    pub username: String,
    pub content: String,
}

impl Summary for Blog {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Implement two things
pub struct SmokeSignal {
    pub creator: String,
    pub content: String,
}

impl Summary for SmokeSignal {
    fn summarize_author(&self) -> String {
        format!("@{}", self.creator)
    }
}

impl fmt::Display for SmokeSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

// Use &impl Summary in the parameter type to allow any type
// that implements Summary
pub fn double_notify_diff_type(item1: &impl Summary, item2: &impl Summary) {
    println!("Two new news articles from different sources! {}, {}", item1.summarize(), item2.summarize());
}

// Use generics and traits to force parameters to use the same type
pub fn double_notify_same_type<T: Summary>(item1: &T, item2: &T) {
    println!("Two new news articles from the same source! {}, {}", item1.summarize(), item2.summarize());
}

// Force an item to implement multiple traits
pub fn notify_multi_impl(item: &(impl Summary + fmt::Display)) {
    println!("Display {}", item);
}

// Same as above but a little cleaner
pub fn notify_multi_impl_2<T: Summary + fmt::Display>(item: &T) {
    println!("Display {}", item);
}

// Same as above but using where
pub fn notify_multi_impl_3<T>(t: &T)
where
    T: Summary + fmt::Display
{
    println!("Display {}", t);
}

// Using where to clean things up
fn some_function_og<T: fmt::Display + Clone, U: Clone + fmt::Debug>(t: &T, u: &U) {}

// This is the same as above, but cleaner
fn some_function_where<T, U>(t: &T, u: &U)
where
    T: fmt::Display + Clone,
    U: Clone + fmt::Debug,
{}


// Returning a type that implements a trait
pub fn returns_summarizable(username: String, content: String) -> impl Summary {
    Tweet {
        username: username,
        content: content,
        reply: false,
        retweet: false,
    }
}
