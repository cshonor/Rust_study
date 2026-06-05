pub mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 写法2 分文件后，路径与书本内嵌写法完全相同
    hosting::add_to_waitlist();
    front_of_house::serving::take_order();
}
