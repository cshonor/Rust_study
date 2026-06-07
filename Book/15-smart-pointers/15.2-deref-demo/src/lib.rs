//! 15.2 Deref / DerefMut demo

use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

/// 教学用：类似 `Box<T>`，数据在结构体内（本节重点在 `Deref` 行为）。
pub struct MyBox<T>(pub T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

pub fn hello(name: &str) {
    println!("Hello, {name}!");
}

/// §二 基础：`*y` ⇔ `*(y.deref())`，coercion `&MyBox<String>` → `&str`
pub fn demo_basic() {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);

    let y = MyBox::new(5);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

/// §三 方法调用自动解引用
pub fn demo_method_call() {
    let b = MyBox::new(String::from("abc"));
    println!("  len = {}（MyBox → &String → str::len）", b.len());
}

/// §四 MutexGuard + Deref
pub fn demo_mutex_guard() {
    let m = Mutex::new(10_i32);
    let guard = m.lock().unwrap();
    println!("  MutexGuard Deref: *guard = {}", *guard);
    println!("  方法调用: guard.abs() = {}", guard.abs());
}

/// §15.2.1 多层 coercion：&Box<MyBox<String>> → &str
pub fn demo_nested_coercion() {
    let nested = Box::new(MyBox::new(String::from("deep")));
    hello(&nested);
    let s: &str = &nested;
    println!("  多层链结果: {s:?}");
}

/// §15.2.1 DerefMut：可变方法 + 索引写
pub fn demo_deref_mut() {
    let mut b = MyBox::new(vec![1, 2, 3]);
    b.push(4);
    (*b)[0] = 99;
    println!("  DerefMut 后 Vec: {:?}", *b);

    let bx = Box::new(String::from("move-out"));
    let s = *bx;
    println!("  Box move-out: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_coercion_to_str() {
        let nested = Box::new(MyBox::new(String::from("chain")));
        fn takes_str(s: &str) -> usize {
            s.len()
        }
        assert_eq!(takes_str(&nested), 5);
    }

    #[test]
    fn deref_mut_vec_push() {
        let mut b = MyBox::new(vec![1]);
        b.push(2);
        assert_eq!(*b, vec![1, 2]);
    }

    #[test]
    fn mybox_string_coercion() {
        let m = MyBox::new(String::from("hi"));
        assert_eq!(hello_as_len(&m), 2);
    }

    fn hello_as_len(s: &str) -> usize {
        s.len()
    }
}
