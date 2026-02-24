// 3.1 变量和可变性 - 示例
// 运行 cargo run 输出如下：

fn main() {
    println!("=== 1. 可变变量 mut ===");
    let mut x = 5;
    println!("The value of x is: {}", x); // 输出: The value of x is: 5
    x = 6;
    println!("The value of x is: {}", x); // 输出: The value of x is: 6

    println!("\n=== 2. 常量 const ===");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS); // 输出: 10800

    println!("\n=== 3. 遮蔽 shadowing ===");
    let x = 5;
    let x = x + 1; // 遮蔽，x 变为 6
    {
        let x = x * 2; // 内部作用域遮蔽，x 变为 12
        println!("The value of x in the inner scope is: {}", x); // 输出: 12
    }
    println!("The value of x is: {}", x); // 输出: 6

    println!("\n=== 4. 遮蔽改变类型 ===");
    let spaces = "   ";
    let spaces = spaces.len(); // 从 &str 变为 usize
    println!("spaces = {} (类型变为 usize)", spaces); // 输出: 3
}

