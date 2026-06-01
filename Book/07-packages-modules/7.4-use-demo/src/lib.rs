mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 重导出：对外隐藏内部模块结构
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub mod results {
    pub mod alpha {
        pub type Result = std::result::Result<&'static str, &'static str>;
    }

    pub mod beta {
        pub type Result = std::result::Result<u32, &'static str>;
    }
}

