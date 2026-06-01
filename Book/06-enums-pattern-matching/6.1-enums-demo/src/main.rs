// 6.1 定义枚举 - 示例

#[derive(Debug, Clone, Copy)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 这里只做最简单的展示：打印 Debug
        println!("Message::call -> {:?}", self);
    }
}

fn route(ip_type: IpAddrKind) {
    println!("routing for {:?}", ip_type);
}

fn main() {
    println!("=== 1) 枚举成员值 ===");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    println!("\n=== 2) 枚举 + 结构体携带数据 ===");
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home = {:?}\nloopback = {:?}", home, loopback);

    println!("\n=== 3) 枚举成员直接携带数据（String） ===");
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));
    println!("home = {:?}\nloopback = {:?}", home, loopback);

    println!("\n=== 4) 不同成员存不同类型/数量数据 ===");
    let home = IpAddrEnum2::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum2::V6(String::from("::1"));
    println!("home = {:?}\nloopback = {:?}", home, loopback);

    println!("\n=== 5) Message 枚举 + 方法 ===");
    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::Move { x: 10, y: 20 };
    m1.call();
    m2.call();

    println!("\n=== 6) Option：Some / None ===");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!(
        "some_number={:?}, some_string={:?}, absent_number={:?}",
        some_number, some_string, absent_number
    );

    // Option<T> 和 T 是不同类型，下面这种写法不允许（示例保留为注释）：
    //
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; // ❌ 编译错误
}

