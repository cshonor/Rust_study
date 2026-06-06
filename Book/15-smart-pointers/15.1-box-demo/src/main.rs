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
    // §二 基础 Box
    let box_num = Box::new(99);
    assert_eq!(*box_num, 99);
    println!("box_num = {box_num}");

    let b = Box::new(5);
    println!("b = {b}");

    // §三 Cons 链表 1 -> 2 -> 3 -> Nil
    let list = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(3, Box::new(Nil))),
        )),
    );
    let link = Cons(5, Box::new(Cons(9, Box::new(Nil))));
    println!("list = {list:?}");
    println!("link = {link:?}");
    println!("list len = {}, sum = {}", list.len(), list.sum());
}
