// 库 crate root：统一对外导出
pub mod math;

pub fn greet(from: &str) -> String {
    format!("hello from library crate: {from}")
}
