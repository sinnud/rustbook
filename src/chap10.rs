pub fn main01(){
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("In main01, The largest number is {}", largest);
}
pub fn main02(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest0(&number_list);
    println!("In main02, The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest0(&number_list);
    println!("The largest number is {}", result);
}
fn largest0(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn main11() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("In main 11, The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
/*
pub fn main12() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn main13() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("in main13, both integer:{:?} both float:{:?} integer and float:{:?}", both_integer, both_float, integer_and_float);
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

pub fn main14() {
    let p = Point { x: 5, y: 10.4 };

    println!("in main14, p.x = {}, p.y={}", p.x(), p.y());
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn main15() {
    let p = Point { x: 5.2, y: 10.4 };

    println!("in main15, distance from origin to {:?} is {}.", p, p.distance_from_origin());
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn main16() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("In main16, p3.x = {}, p3.y = {}", p3.x(), p3.y());
}

// should be in lib.rs, but try put it here
pub trait Summary {
    //fn summarize(&self) -> String;
    fn summarize_author(&self) -> String{
        format!("...to be continued...")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
//pub fn notify(item: &impl Summary) {
//pub fn notify<T: Summary>(item: &T) {
pub fn notify<T>(item: &T) 
where T: Summary
{
    println!("Breaking news! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String{
        format!("@{}", self.author)
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
pub fn main21(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle{
        headline: String::from("Adaptive Method"),
        location: String::from("Hawkeye Court"),
        author  : String::from("Luke"),
        content : String::from("abcdefg"),
    };

    println!("in main21, 1 new tweet: {}", tweet.summarize());
    println!("           1 new article: {}", article.summarize());
    notify(&tweet);
    notify(&article);
}
pub fn main22() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("in main22, The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("           The largest char is {}", result);
}
fn largest<T>(list: &[T]) -> T
where T: PartialOrd + Copy
{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
pub fn main31() {
    {
        let r;

        // {
            let x = 5;
            r = &x;
        // }

        println!("in main31, r: {}", r);
    }
}

pub fn main32() {
    let string1 = String::from("abcd");
    //let result;
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("in main32, The longest string is {}", result);
    }
}
//fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
#[derive(Debug)]
struct ImportantExcerpt {
    part: String,
}
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

pub fn main33() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence.to_string(),
        // part: first_sentence,
    };
    println!("in main33, The struct example is {:?}", i);
    let thisint=i.level();
    let p=i.announce_and_return_part(&"This is from Luke");
    println!("           The integer is {}, return part is {}", thisint, p);
}
//impl<'a> ImportantExcerpt<'a> {
impl ImportantExcerpt {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> String {
        println!("Attention please: {}", announcement);
        self.part.clone()
    }
}
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main34() {
    let string1 = String::from("abcd");
    //let result;
    {
        let string2 = "xyz";
        let result = longest_with_an_announcement(string1.as_str(), string2, "in struct");
        println!("in main34, The longest string is {}", result);
    }
}