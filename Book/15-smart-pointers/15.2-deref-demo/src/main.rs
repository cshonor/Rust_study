// 15.2 Deref demo
//   cargo run           — 基础 + coercion + 方法调用
//   cargo run -- guards — MutexGuard
//   cargo run -- nested  — 多层 coercion
//   cargo run -- mut     — DerefMut + Box move-out

use deref_demo::{demo_basic, demo_deref_mut, demo_method_call, demo_mutex_guard, demo_nested_coercion};

fn main() {
    let arg = std::env::args().nth(1);
    let mode = arg.as_deref().unwrap_or("full");

    if mode == "guards" {
        println!("=== 15.2 §四 MutexGuard + Deref ===");
        demo_mutex_guard();
        println!("\nok: guards demo 完成");
        return;
    }

    if mode == "nested" {
        println!("=== 15.2.1 多层 coercion 链 ===");
        demo_nested_coercion();
        println!("\nok: nested demo 完成");
        return;
    }

    if mode == "mut" {
        println!("=== 15.2.1 DerefMut + Box move-out ===");
        demo_deref_mut();
        println!("\nok: mut demo 完成");
        return;
    }

    println!("=== 15.2 基础：* / deref / coercion ===");
    demo_basic();

    println!("\n=== 15.2 §三 方法调用自动解引用 ===");
    demo_method_call();

    println!("\nok: deref demo 完成（-- guards | -- nested | -- mut）");
}
