use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 示例 15-18 / 15-19：共享尾列表 a，并观察 strong_count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("c = {c:?}");
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("b = {b:?}");
}
