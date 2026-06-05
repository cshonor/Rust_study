// 8.1 Vec<T> 完整 demo

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("=== 1) 三种创建 + 类型推断 ===");
    let empty: Vec<i32> = Vec::new(); // 空 Vec 须标注（或靠后续 push 推导）
    println!("Vec::new() len={} cap={}", empty.len(), empty.capacity());

    let mut inferred = Vec::new();
    inferred.push(5i32); // 第一次 push 反向锁定 Vec<i32>，不必标注
    println!("Vec::new()+push(5i32) = {:?}", inferred);

    let v = vec![1, 2, 3]; // 有初始元素，自动推断 Vec<i32>
    let zeros = vec![0; 5];
    println!("vec![1,2,3] = {:?}", v);
    println!("vec![0;5] = {:?}", zeros);

    let mut pre = Vec::with_capacity(10);
    pre.push(1);
    println!(
        "with_capacity(10) after 1 push: len={} cap={}",
        pre.len(),
        pre.capacity()
    );

    println!("\n=== 2) push 更新（已标注 Vec<i32>）===");
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    println!("pushed = {:?}", v);

    println!("\n=== 3) 作用域 Drop ===");
    {
        let _v = vec![1, 2, 3];
    }
    println!("块结束，Vec 已 Drop");

    println!("\n=== 4) [] vs get ===");
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("&v[2] = {}", third);
    match v.get(2) {
        Some(x) => println!("get(2) = {}", x),
        None => println!("get(2) = None"),
    }
    assert!(v.get(100).is_none());
    if let Some(x) = v.get_mut(1) {
        *x += 10;
    }
    println!("after get_mut(1)+10 = {:?}", v);

    println!("\n=== 5) 借用规则 + push 规避 ===");
    let mut v = vec![1, 2, 3, 4, 5];
    {
        let first = &v[0];
        println!("first = {}", first);
        // v.push(6); // ❌ E0502：引用存在时不能 push（可能 realloc）
    }
    v.push(6);
    println!("after scope, push(6) ok = {:?}", v);

    println!("\n=== 6) pop / remove / clear / shrink_to_fit ===");
    let mut v = vec![1, 2, 3];
    println!("pop = {:?}", v.pop());
    v.remove(0);
    println!("after remove(0) = {:?}", v);
    v.clear();
    println!("after clear: len={} cap={}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("after shrink_to_fit: len={} cap={}", v.len(), v.capacity());

    println!("\n=== 7) 遍历 ===");
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

    println!("\n=== 8) enum 多类型 ===");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row = {:?}", row);
    println!("\nok: Vec 创建/读取/借用/拓展方法");
}
