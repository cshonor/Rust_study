// 库 crate root：对外 API 写在这里
pub fn greet(from: &str) -> String {
    format!("hello from library crate: {from}")
}
