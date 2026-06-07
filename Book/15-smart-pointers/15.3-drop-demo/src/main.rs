// 15.3 Drop demo
//   cargo run           — 作用域 + LIFO 顺序
//   cargo run -- early  — std::mem::drop
//   cargo run -- nested — Outer/Inner + Vec 正序
//   cargo run -- guard  — MutexGuard RAII
//   cargo run -- manual — ManuallyDrop

use drop_demo::{
    demo_manually_drop, demo_mem_drop_early, demo_mutex_guard_drop, demo_nested_drop,
    demo_scope_and_order, demo_vec_drop_order,
};

fn main() {
    let arg = std::env::args().nth(1);
    let mode = arg.as_deref().unwrap_or("full");

    if mode == "early" {
        println!("=== 15.3 §三 mem::drop 提前释放 ===\n");
        demo_mem_drop_early();
        println!("\nok: early demo 完成");
        return;
    }

    if mode == "nested" {
        println!("=== 15.3.1 嵌套 + Vec drop 顺序 ===\n");
        demo_nested_drop();
        println!();
        demo_vec_drop_order();
        println!("\nok: nested demo 完成");
        return;
    }

    if mode == "guard" {
        println!("=== 15.3 §四 MutexGuard RAII ===\n");
        demo_mutex_guard_drop();
        println!("\nok: guard demo 完成");
        return;
    }

    if mode == "manual" {
        println!("=== 15.3.1 ManuallyDrop ===\n");
        demo_manually_drop();
        println!("\nok: manual demo 完成");
        return;
    }

    println!("=== 15.3 作用域自动 drop + LIFO ===\n");
    demo_scope_and_order();
    println!("\nok: drop demo 完成（-- early | -- nested | -- guard | -- manual）");
}
