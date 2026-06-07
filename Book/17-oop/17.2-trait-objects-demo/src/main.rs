// 17.2 Trait 对象 demo
//   cargo run           — GUI Draw 完整 demo（含扩展 SelectBox）
//   cargo run -- compare — 枚举 vs 泛型 vs trait 对象
//   cargo run -- safety  — 对象安全规则

use trait_objects_gui_demo::{
    demo_enum_vs_dyn_notes, demo_generic_vs_dyn_notes, demo_gui_trait_objects,
    demo_object_safety_notes, Draw, Screen,
};

/// 使用者在 main 中扩展的类型 — 无需修改 Screen / Draw 库代码
struct SelectBox;

impl Draw for SelectBox {
    fn draw(&self) {
        println!("  绘制下拉选择框");
    }
}

fn main() {
    let arg = std::env::args().nth(1);
    let mode = arg.as_deref().unwrap_or("full");

    if mode == "compare" {
        println!("=== 17.2 枚举 vs 泛型 vs Trait 对象 ===\n");
        demo_enum_vs_dyn_notes();
        demo_generic_vs_dyn_notes();
        println!("\nok: compare demo 完成");
        return;
    }

    if mode == "safety" {
        println!("=== 17.2 对象安全（Object Safe）===\n");
        demo_object_safety_notes();
        println!("\nok: safety demo 完成");
        return;
    }

    println!("=== 17.2 GUI Trait 对象 Demo ===\n");
    demo_gui_trait_objects();

    // 扩展：新增 SelectBox，不改 Screen / Draw
    println!("\n--- 扩展 SelectBox（不修改库代码）---");
    let mut screen = Screen::new();
    screen.add_component(Box::new(trait_objects_gui_demo::Button {
        label: "确定".into(),
    }));
    screen.add_component(Box::new(trait_objects_gui_demo::TextField {
        content: "请输入内容".into(),
    }));
    screen.add_component(Box::new(SelectBox));
    screen.run();

    println!("\nok: trait objects demo 完成（-- compare / -- safety）");
}
