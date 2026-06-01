//! 示例 18-8～18-10：可反驳 / 不可反驳模式与 `let` vs `if let`。

fn main() {
    let some_option_value = Some(3u8);

    // 示例 18-9：`if let` 允许可反驳模式；不匹配则跳过。
    if let Some(x) = some_option_value {
        println!("if let Some(x) => {x}");
    }

    // 示例 18-8：下面一行不能编译（`None` 未覆盖），故注释掉：
    // let Some(x) = some_option_value;
    // error[E0005]: refutable pattern in local binding: `None` not covered

    irrefutable_if_let_example();
}

/// 示例 18-10：`if let` 使用不可反驳模式会触发警告；此处仅作演示并允许告警以便 `cargo build` 通过。
#[allow(irrefutable_let_patterns)]
fn irrefutable_if_let_example() {
    if let x = 5 {
        println!("if let x = 5 => {x} (prefer plain `let` in real code)");
    }
}
