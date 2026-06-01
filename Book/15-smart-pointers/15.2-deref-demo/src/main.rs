use std::ops::Deref;

/// 教学用：类似 `Box<T>`，但数据就在结构体里（本节重点在 `Deref` 行为）。
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // 示例 15-6：常规引用
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 示例 15-7：`Box<T>`
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);

    // 示例 15-9 / 15-10：`MyBox` + `Deref`，`*y` 等价于 `*(y.deref())`
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    // 示例 15-12：解引用强制转换 `&MyBox<String>` → `&str`
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 示例 15-13：无强制转换时需写的显式形式
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
