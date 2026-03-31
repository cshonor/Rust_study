// 3.2 数据类型 - 示例

fn main() {
    println!("=== 1. 类型标注 ===");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess); // 输出: 42

    println!("\n=== 2. 整型字面量 ===");
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("decimal={}, hex={}, octal={}, binary={}, byte={}", decimal, hex, octal, binary, byte);
    // 输出: 98222, 255, 63, 240, 65

    println!("\n=== 3. 浮点类型 ===");
    let x = 2.0; // f64
    let y: f32 = 3.0;
    println!("x(f64)={}, y(f32)={}", x, y); // 输出: 2, 3

    println!("\n=== 4. 数字运算 ===");
    let sum = 5 + 10;           // 15
    let difference = 95.5 - 4.3; // 91.2
    let product = 4 * 30;       // 120
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;        // 0 整数除法向下取整
    let remainder = 43 % 5;     // 3
    println!("sum={}, diff={}, product={}, quotient={:.2}, floored={}, remainder={}",
        sum, difference, product, quotient, floored, remainder);

    println!("\n=== 5. 布尔类型 ===");
    let t = true;
    let f: bool = false;
    println!("t={}, f={}", t, f); // 输出: true, false

    println!("\n=== 6. 字符类型 char ===");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c={}, z={}, emoji={}", c, z, heart_eyed_cat); // z, ℤ, 😻

    println!("\n=== 7. 元组 tuple ===");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // 解构
    println!("解构: x={}, y={}, z={}", x, y, z); // 500, 6.4, 1
    println!("索引: tup.0={}, tup.1={}, tup.2={}", tup.0, tup.1, tup.2); // 同上

    println!("\n=== 8. 数组 array ===");
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];  // [3, 3, 3, 3, 3]
    println!("a[0]={}, a[1]={}", a[0], a[1]); // 1, 2
    println!("c = {:?}", c);  // [3, 3, 3, 3, 3]

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    println!("months[0]={}", months[0]); // Jan
}

