mod ref_cycle;
mod weak_tree;

fn main() {
    println!("=== 引用循环（15-26）===\n");
    ref_cycle::run();

    println!("\n=== Weak 父指针（15-29）===\n");
    weak_tree::run();
}
