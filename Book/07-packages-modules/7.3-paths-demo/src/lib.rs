mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            // do something
        }
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    impl Breakfast {
        pub fn seasonal_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    back_of_house::fix_incorrect_order();
}

pub use back_of_house::{Appetizer, Breakfast};

