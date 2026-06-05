pub mod front_of_house;
pub mod domain;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 写法2 分文件后，路径与书本内嵌写法完全相同
    hosting::add_to_waitlist();
    front_of_house::serving::take_order();
}

pub fn demo_domain_mod_use() {
    domain::check();
    let _u = domain::UserEntity::new(); // pub use 重导出
}
