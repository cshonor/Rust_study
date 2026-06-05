fn main() {
    split_modules_demo::eat_at_restaurant();
    split_modules_demo::hosting::add_to_waitlist();
    split_modules_demo::demo_domain_mod_use();
    println!("ok: front_of_house 分文件 + domain mod.rs 里 mod/use/pub use");
}

