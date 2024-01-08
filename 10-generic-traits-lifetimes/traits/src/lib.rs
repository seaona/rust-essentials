use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}

// declaring a trait
// each type implemeting this trait must provide its own custom behavior for the body of the method summarize
pub trait Summary {
    fn summarize(&self) -> String; 
}

// implementing a trait on a type
pub struct NewsArticle {
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

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// Default implementations
pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// we could implement NewsArticle without defining summarize, but we can still call it
// article.summarize()

let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
    ),
};

println!("New article available! {}", article.summarize());


// default implementation calling other methods
pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    // we can call any methods on item that come from the Summary trait
    println!("Breaking news! {}", item.summarize());
}

// Trait bound
pub fn notify(item: &impl Summary) {...}
pub fn notify<T: Summary>(item: &T) {...}

    // params with different types: syntatic sugar
pub fn notify(item1: &impl Summary, item2: &impl Summary) {...}

    // params with the same type: use trait bound
pub fn notify<T: Summary>(item1: &T, item2: &T)

    // multiple trait bounds with + syntax
pub fn notify(item: &impl(Summary + Display)) {}
pub fn notify <T: Summary + Display>(item: &T)

    // clearer trait bounds with 'where' clause
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &Y) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    t: Display + Clone,
    U: Clone + Debug,
{}

    // returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_books"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

    // using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

// Pair<T> always implements the new function, to return a new instance of Pair<T>
impl<T> Pair<T> {
    fn new(x: T, y: T) -> self { 
        Self { x, y }
    }
}

// Pair<T> only implements the cmpl_display method if its inner type T implements the PartialOrd trait
// that enables the comparision and the Dsiplay trait that enables printing
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

    // blanket implementations
impl<T: Display> ToString for T {...}
let s= 3.to_string();