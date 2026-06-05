// main.rs = bin crate root（另一棵独立模块树，与 lib 互不干扰）
fn main() {
    modules_privacy_demo::eat_at_restaurant();
    println!("ok: bin crate 通过 lib 的 pub API 调用模块树内函数");
}
