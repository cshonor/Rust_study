pub mod math;

// 可选：再导出，外部 use 路径更短
// pub use math::add;

pub fn greet(from: &str) -> String {
    format!("hello from library crate: {from}")
}
