// 10.2 trait：定义共享的行为 - 示例

use std::fmt::{Debug, Display};

// --- 1) 定义 trait ---
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// --- 2) 为类型实现 trait ---
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize 使用默认实现
}

// --- 3) trait 作为参数 ---
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// --- 4) where 从句 ---
fn print_two<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("t = {}, u = {:?}", t, u);
}

// --- 5) largest：用 trait bound 修复 ---
fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 返回引用版本：不需要 Copy/Clone
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// --- 6) 条件实现 Pair<T> ---
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
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
    println!("=== 1) Summary trait + 实现/默认实现 ===");
    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".into(),
        location: "Pittsburgh, PA, USA".into(),
        author: "Iceburgh".into(),
        content: "The Pittsburgh Penguins once again are the best hockey team in the NHL.".into(),
    };

    let tweet = Tweet {
        username: "horse_ebooks".into(),
        content: "of course, as you probably already know, people".into(),
        reply: false,
        retweet: false,
    };

    println!("article.summarize() = {}", article.summarize());
    println!("tweet.summarize()   = {}", tweet.summarize()); // 默认实现

    println!("\n=== 2) trait 作为参数 ===");
    notify(&article);
    notify(&tweet);
    notify_bound(&tweet);

    println!("\n=== 3) where 从句 ===");
    print_two("hello".to_string(), vec![1, 2, 3]);

    println!("\n=== 4) largest：Copy vs 引用 ===");
    let nums = vec![34, 50, 25, 100, 65];
    println!("largest_copy(nums) = {}", largest_copy(&nums));

    let words = vec!["aaa".to_string(), "zz".to_string(), "mmmm".to_string()];
    println!("largest_ref(words) = {}", largest_ref(&words));

    println!("\n=== 5) 条件实现 Pair<T> ===");
    let p = Pair::new(3, 7);
    p.cmp_display();
}

