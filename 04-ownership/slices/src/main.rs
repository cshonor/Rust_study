// 4.3 切片 Slice - 示例

fn main() {
    println!("=== 1. 字符串 slice 语法 ===");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello = '{}', world = '{}'", hello, world);

    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];  // 等价
    let slice3 = &s[3..];
    let slice4 = &s[..];   // 整个字符串
    println!("[0..2]='{}', [..2]='{}', [3..]='{}', [..]='{}'", slice1, slice2, slice3, slice4);

    println!("\n=== 2. first_word 返回 slice ===");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word: '{}'", word);

    println!("\n=== 3. &str 参数：更通用的 API ===");
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("from String slice: '{}'", word);

    let my_string_literal = "hello world";
    let word = first_word(my_string_literal);  // 字面量本身就是 &str
    println!("from literal: '{}'", word);

    println!("\n=== 4. 数组 slice ===");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("&a[1..3] = {:?}", slice);  // [2, 3]
}

/// 返回第一个单词的 slice
/// 参数用 &str 可接受 String、&str、字面量
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]  // 无空格则返回整个字符串
}

