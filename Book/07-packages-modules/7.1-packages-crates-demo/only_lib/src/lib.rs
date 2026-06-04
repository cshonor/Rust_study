// 纯 library crate：无 main，cargo build 产出库，不能 cargo run
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
