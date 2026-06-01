fn main() {
    split_modules_demo::eat_at_restaurant();
    split_modules_demo::hosting::add_to_waitlist();
    println!("ok: modules loaded from multiple files");
}

