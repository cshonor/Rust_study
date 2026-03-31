// 5.1 定义并实例化结构体 - 示例

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // 字段初始化简写：变量名与字段名相同
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 元组结构体：有名字但字段无名字
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// 类单元结构体：没有字段
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    println!("=== 1) 实例化 + 访问字段 ===");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1.email = {}", user1.email);

    println!("\n=== 2) 修改字段（实例必须 mut）===");
    let mut user2 = build_user(String::from("a@ex.com"), String::from("alice"));
    user2.email = String::from("alice@ex.com");
    println!("user2 = {:?}", user2);

    println!("\n=== 3) 结构体更新语法（注意 move）===");
    let user3 = User {
        email: String::from("another@example.com"),
        // .. 会把剩余字段从 user2 移动/拷贝过来：
        // - username: String 会被 move
        // - active(bool), sign_in_count(u64) 会 Copy
        ..user2
    };
    println!("user3 = {:?}", user3);

    // 这里 user2 已经不能再用（username 被 move），下面代码会报错：
    // println!("{:?}", user2);

    println!("\n=== 4) 元组结构体 ===");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black.0 = {}, origin.1 = {}", black.0, origin.1);
    println!("black = {:?}, origin = {:?}", black, origin);

    println!("\n=== 5) 类单元结构体 ===");
    let subject = AlwaysEqual;
    println!("subject = {:?}", subject);
}

