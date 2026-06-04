pub mod calc;

pub use calc::mul;

pub fn greet(from: &str) -> String {
    format!("hello from library crate: {from}")
}
