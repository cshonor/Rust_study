//! 7.2 模块树 demo — lib.rs = lib crate root = 模块树顶层 `crate`
//!
//! 模块树：
//! ```text
//! crate
//!  └── front_of_house
//!      └── hosting → add_to_waitlist (pub) / seat_at_table (私有)
//! ```

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            seat_at_table(); // 同模块内调私有函数
        }

        fn seat_at_table() {}
    }

    mod serving {}
}

/// 对外公开 API：crate 外（含 main bin）只应调这类函数
pub fn eat_at_restaurant() {
    // 相对路径（当前在 crate 根模块）
    front_of_house::hosting::add_to_waitlist();

    // 等价绝对路径：
    // crate::front_of_house::hosting::add_to_waitlist();
}
