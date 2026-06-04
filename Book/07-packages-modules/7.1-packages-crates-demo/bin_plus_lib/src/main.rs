// 二进制 crate root：同 package 的库 crate 自动可用
fn main() {
    let msg = bin_plus_lib_demo::greet("src/main.rs (binary crate)");
    println!("{msg}");
}
