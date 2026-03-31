// 8.1 Vec<T> / vector - 示例

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("=== 1) 新建 vector ===");
    let v: Vec<i32> = Vec::new();
    println!("v (empty) = {:?}", v);

    let v = vec![1, 2, 3];
    println!("v (vec!) = {:?}", v);

    println!("\n=== 2) push 更新 ===");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v (pushed) = {:?}", v);

    println!("\n=== 3) 读取元素：索引 vs get ===");
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 越界演示（保留注释）
    // let does_not_exist = &v[100];      // ❌ panic
    // let does_not_exist = v.get(100);   // ✅ None

    println!("\n=== 4) 借用规则：引用存在时不能 push ===");
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("first = {}", first);
    // v.push(6); // ❌ error[E0502]

    println!("\n=== 5) 遍历 ===");
    let v = vec![100, 32, 57];
    for i in &v {
        print!("{} ", i);
    }
    println!();

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("after +50 = {:?}", v);

    println!("\n=== 6) 用枚举存多类型 ===");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row = {:?}", row);
}

