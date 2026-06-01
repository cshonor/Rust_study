// 10.3 生命周期与引用有效性 - 示例

use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 返回 s 的第一个 UTF-8 标量（演示 `'a`：返回引用与入参同寿命）
fn get_first<'a>(s: &'a str) -> &'a str {
    let end = s
        .char_indices()
        .nth(1)
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    &s[0..end]
}

// fn bad<'a>() -> &'a str {
//     let s = String::from("hello");
//     &s
// }

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("=== 0) get_first：返回引用与入参同 'a ===");
    let s = String::from("中文abc");
    let first = get_first(&s);
    println!("get_first(\"中文abc\") = {:?}", first);

    println!("\n=== 1) longest ===");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("longest = {}", result);

    println!("\n=== 2) 不同作用域下的 longest（有效示例） ===");
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("longest = {}", result);
    }

    // 悬垂引用示例（不能编译，保留注释）：
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("{}", result); // ❌ string2 已释放

    println!("\n=== 3) 结构体中存引用 ImportantExcerpt<'a> ===");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("level = {}", i.level());
    println!(
        "part = {}",
        i.announce_and_return_part("hello lifetimes")
    );

    println!("\n=== 4) 'static ===");
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    println!("\n=== 5) 泛型 + trait bound + 生命周期 ===");
    let x = "short";
    let y = "a bit longer";
    let r = longest_with_an_announcement(x, y, 123);
    println!("result = {}", r);
}

