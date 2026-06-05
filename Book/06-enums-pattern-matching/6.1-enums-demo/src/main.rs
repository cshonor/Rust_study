// 6.1 定义枚举 — 完整 demo

#[derive(Debug, Clone, Copy)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrLegacy {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct User {
    name: String,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Test {
    Item1(u8, i32, String),
    Item2(User, Point),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("Message::call");
    }

    fn describe(&self) -> String {
        match self {
            Message::Quit => "退出".into(),
            Message::Move { x, y } => format!("坐标 {} {}", x, y),
            Message::Write(s) => format!("写入: {}", s),
            Message::ChangeColor(r, g, b) => format!("rgb {} {} {}", r, g, b),
        }
    }
}

fn route(ip_type: IpAddrKind) {
    println!("routing {:?}", ip_type);
}

fn main() {
    println!("=== 1) 单元变体 ===");
    let four = IpAddrKind::V4;
    route(four);

    println!("\n=== 2) 老式 enum + struct（不推荐）===");
    let legacy = IpAddrLegacy {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("legacy = {:?}", legacy);

    println!("\n=== 3) 元组变体 () — 多类型 / 嵌 struct ===");
    let localhost = IpAddr::V4(192, 168, 1, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("IPv4 {:?}, IPv6 {:?}", localhost, loopback);

    let t1 = Test::Item1(1, 42, String::from("hi"));
    let t2 = Test::Item2(
        User {
            name: String::from("张三"),
        },
        Point { x: 10, y: 20 },
    );
    println!("Item1 {:?}, Item2 {:?}", t1, t2);

    println!("\n=== 4) 标准库 std::net::IpAddr ===");
    use std::net::{IpAddr as StdIpAddr, Ipv4Addr};
    let ip: StdIpAddr = StdIpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("is_loopback = {}", ip.is_loopback());

    println!("\n=== 5) Message 四种变体 + impl ===");
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
    m1.call();
    for m in [&m2, &m3, &m4] {
        println!("  {}", m.describe());
    }

    println!("\n=== 6) Option<T> ===");
    let x = Some(10);
    let empty: Option<i32> = None;
    println!(
        "is_some={} is_none={} unwrap_or={}",
        x.is_some(),
        empty.is_none(),
        empty.unwrap_or(0)
    );
    // let sum = 5 + x; // ❌ Option<i32> 与 i32 不能混用

    println!("\nok: enum 变体 · impl · Option · match 预告");
}
