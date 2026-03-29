// 4.1 什么是所有权 - 示例

fn main() {
    println!("=== 1. 移动 (Move) ===");
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动，s1 不再有效
    println!("s2 = {}", s2); // s2 有效

    println!("\n=== 2. 克隆 (Clone) ===");
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝堆数据
    println!("s1 = {}, s2 = {}", s1, s2); // 两者都有效

    println!("\n=== 3. Copy 类型 ===");
    let x = 5;
    let y = x; // 拷贝，不是移动
    println!("x = {}, y = {}", x, y); // 两者都有效

    println!("\n=== 4. 所有权与函数 ===");
    let s = String::from("hello");
    takes_ownership(s); // s 被移动进函数
                        // s 不再有效

    let x = 5;
    makes_copy(x); // x 被拷贝
    println!("main: x 仍有效 = {}", x); // x 仍有效

    println!("\n=== 5. 返回值转移所有权 ===");
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 被移动，返回值给 s3
    println!("s1 = {}, s3 = {}", s1, s3);

    println!("\n=== 6. 用元组返回所有权 ===");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
} // some_string 离开作用域，drop 被调用

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
} // some_integer 离开作用域，无特殊操作（栈上）

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回并移动给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 原样返回，移动给调用者
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 返回 s，把所有权还回去
}

