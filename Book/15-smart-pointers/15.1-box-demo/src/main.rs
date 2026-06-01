#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

impl List {
    fn len(&self) -> usize {
        match self {
            Cons(_, rest) => 1 + rest.len(),
            Nil => 0,
        }
    }

    fn sum(&self) -> i32 {
        match self {
            Cons(head, rest) => head + rest.sum(),
            Nil => 0,
        }
    }
}

fn main() {
    // 示例 15-1：堆上存放 i32
    let b = Box::new(5);
    println!("b = {b}");

    // 示例 15-5：用 Box 打破递归，构造 cons list 1 -> 2 -> 3 -> Nil
    let list = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(3, Box::new(Nil))),
        )),
    );
    println!("list = {list:?}");
    println!("len = {}", list.len());
    println!("sum = {}", list.sum());
}
