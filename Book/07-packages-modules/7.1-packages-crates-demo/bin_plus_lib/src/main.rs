use bin_plus_lib_demo::calc::mul;

fn main() {
    println!("{}", mul(5, 6));
    let msg = bin_plus_lib_demo::greet("src/main.rs (binary crate)");
    println!("{msg}");
}
