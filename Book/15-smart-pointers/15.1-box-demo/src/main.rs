// 15.1 Box demo
//   cargo run        — 基础 Box + Cons 链表
//   cargo run -- drop — Box 出作用域释堆

use box_demo::{demo_box_basics, demo_box_drop_scope, demo_cons_list};

fn main() {
    let arg = std::env::args().nth(1);
    let mode = arg.as_deref().unwrap_or("full");

    if mode == "drop" {
        println!("=== 15.1 §四 Box + Drop 释堆 ===\n");
        demo_box_drop_scope();
        println!("\nok: drop demo 完成");
        return;
    }

    println!("=== 15.1 §三 基础 Box + Deref ===\n");
    demo_box_basics();

    println!("\n=== 15.1 §二 Cons 链表 1→2→3 ===\n");
    demo_cons_list();

    println!("\nok: box demo 完成（-- drop）");
}
