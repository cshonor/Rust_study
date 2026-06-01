mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            // internal details
            seat_at_table();
        }

        fn seat_at_table() {}
    }

    mod serving {}
}

pub fn eat_at_restaurant() {
    // 通过路径访问模块中的公开函数
    front_of_house::hosting::add_to_waitlist();
}

