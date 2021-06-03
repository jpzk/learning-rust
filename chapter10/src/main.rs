// generics


use std::cmp::PartialOrd;
use std::marker::Copy;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { 
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub trait Summary {
    fn summarize(&self) -> String { 
        let author = self.summarize_author();
        format!("Read more here from {}", author)
    }

    fn summarize_author(&self) -> String;
}
// implements Summary (short form)
//pub fn notify(item: &impl Summary) { 
//    println!("Breaking news! {} ", item.summarize());
//}

// for adding multiple trait bounds with + 
use std::fmt::Display;
use std::fmt::Debug;

pub fn notify<T: Summary + Display>(item: &T) { 
    println!("Breaking news! {} ", item.summarize()); 
}

// or 
pub fn notify2(item: &(impl Summary + Display)) { 
    println!("Breaking news! {} ", item.summarize()); 
}

// or for beauty with where clause after return type
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug 
{
    0
}

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

//pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// for different traits

//pub fn notify<T: Summary>(item1: &T, item2: &T) {
// for the smame traits


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String { 
        format!("author: {}", self.author)
    } 
} // uses default impl

pub struct Tweet {
    pub username: String, 
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String { 
        format!("author: {}", self.username)
    }
}

struct Point<T> { 
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> Point<T> { 
    fn x(&self) -> &T { 
        &self.x
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Pair<T> { 
    x: T,
    y: T
}

impl<T> Pair<T> { 
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}
impl<T: Display + PartialOrd> Pair<T> { 
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let p = Point { x: 51.1, y: 10.0};
    println!("p.x = {}", p.x());
    println!("distance from origin: {}", p.distance_from_origin());

    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from(
            "of course..."
        ),
        reply: false,
        retweet: false
    };
    println!("1 new tweet {}", tweet.summarize());
    println!("Hello, world!");
}

