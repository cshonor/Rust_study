//! Item 5/6：Deref 强制转换 + derive_more newtype

use derive_more::{Add, Display, From};
use std::ops::Deref;

/// derive_more：减少 `.0` 样板
#[derive(Debug, Clone, Copy, PartialEq, From, Add, Display)]
struct Meters(u32);

/// 手动 Deref：透明访问内部（Item 5 — 方法解析会尝试 deref）
struct Wrapper(String);

impl Deref for Wrapper {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_len(s: &str) {
    println!("len = {}", s.len());
}

fn main() {
    let a = Meters(3);
    let b = Meters(5);
    println!("distance = {} m", a + b);

    let w = Wrapper("hello".into());
    print_len(&w); // &Wrapper → &str via Deref coercion
    println!("deref str: {}", w.to_uppercase());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meters_add() {
        assert_eq!(Meters(1) + Meters(2), Meters(3));
    }

    #[test]
    fn wrapper_coerces_to_str() {
        let w = Wrapper("ab".into());
        assert_eq!(w.len(), 2);
    }
}
