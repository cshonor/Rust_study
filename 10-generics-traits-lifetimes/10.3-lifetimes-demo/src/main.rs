// 10.3 生命周期与引用有效性 - 示例

use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_first<'a>(s: &'a str) -> &'a str {
    let end = s
        .char_indices()
        .nth(1)
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    &s[0..end]
}

fn borrow_num<'a>(data: &'a i32) -> &'a i32 {
    data
}

fn cmp<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if *x > *y { x } else { y }
}

// 空函数体：签名契约仍须满足，与函数内用不用形参无关
#[allow(dead_code)]
fn demo<'a>(_x: &'a i32, _y: &'a i32) {}

// 笔记反例：demo(&outer, &inner) — 契约 vs 实际寿命不匹配
#[allow(dead_code)]
fn same_group<'a>(_p1: &'a i32, _p2: &'a i32) {}

// 笔记反例：use_both(&long_live, &short_live) 同 'a 但 long 超组上限 → 编译报错
#[allow(dead_code)]
fn use_both<'a>(a: &'a i32, b: &'a i32) {
    println!("{} {}", a, b);
}

fn print_both<'a>(x: &'a i32, y: &'a i32) {
    println!("{} {}", x, y);
}

// 笔记反例：print_two(&outer, &inner) 同 'a 但 outer 比 inner 活得长
#[allow(dead_code)]
fn print_two<'a>(x: &'a i32, y: &'a i32) {
    println!("{} {}", x, y);
}

fn take_same<'a>(a: &'a i32, b: &'a i32) {
    println!("{} {}", a, b);
}

fn two_group<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("{} {}", x, y);
}

fn triple<'a, 'b, 'c>(x: &'a i32, y: &'b i32, z: &'c i32) {
    println!("{} {} {}", x, y, z);
}

fn pick_one<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32 {
    x
}

fn return_two<'a, 'b>(x: &'a i32, y: &'b i32) -> (&'a i32, &'b i32) {
    (x, y)
}

fn pick_any<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if *x > *y { x } else { y }
}

fn get_static_str() -> &'static str {
    let s = "常驻文本";
    s
}

// fn bad<'a>() -> &'a i32 {
//     let tmp = 20;
//     &tmp
// }

// fn bad_str<'a>() -> &'a str {
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
    println!("=== 0) 变量生命周期 vs 'a 标注 ===");
    let num = 10;
    let r = borrow_num(&num);
    println!("borrow<'a>: r = {}", r);

    println!("\n=== 0.5) &'static str：变量消失，数据不 drop ===");
    {
        let s = "块内字面量";
        println!("块内 s = {}", s);
    }
    println!("返回 &'static str: {}", get_static_str());

    let s: &'static str = "abc";
    println!("s: &'static str → 只读段 \"{}\" 地址 {:p}", s, s);

    println!("\n=== 0.8) 最晚可用时间 & 'a 分组 ===");
    let v1 = 10;
    let v2 = 20; // 同块 → &v1、&v2 最晚可用时间都是 main 末尾
    println!("cmp: {}", cmp(&v1, &v2));

    let num = 99;
    take_same(&num, &num); // 同一变量，只是特例

    let outer = 100;
    {
        let inner = 200;
        two_group(&outer, &inner);
        // demo(&outer, &inner);   // ❌ 空函数也报错：契约不匹配
        // same_group(&outer, &inner);
        {
            let a = 1;
            let b = 2;
            print_both(&a, &b);
        }
    }
    println!("出内层块后 outer 原变量仍可用: {}", outer); // ✅ 限制的是引用，不是 outer

    triple(&v1, &v2, &num);
    println!("pick_one: {}", pick_one(&v1, &v2));
    let (rx, ry) = return_two(&v1, &v2);
    println!("return_two: ({}, {})", rx, ry);
    println!("pick_any max: {}", pick_any(&v1, &v2));

    println!("\n=== 1) get_first：返回引用与入参同 'a ===");
    let s = String::from("中文abc");
    let first = get_first(&s);
    println!("get_first(\"中文abc\") = {:?}", first);

    println!("\n=== 2) longest ===");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("longest = {}", result);

    println!("\n=== 3) 不同作用域下的 longest ===");
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("longest = {}", result);
    }

    println!("\n=== 4) 结构体 ImportantExcerpt<'a> ===");
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

    println!("\n=== 5) 'static ===");
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    println!("\n=== 6) 泛型 + trait bound + 生命周期 ===");
    let x = "short";
    let y = "a bit longer";
    let r = longest_with_an_announcement(x, y, 123);
    println!("result = {}", r);
}
