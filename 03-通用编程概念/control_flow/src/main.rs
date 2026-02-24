// 3.5 控制流 - 示例

fn main() {
    println!("=== 1. if / else ===");
    let number = 3;
    if number < 5 {
        println!("condition was true"); // 输出
    } else {
        println!("condition was false");
    }

    println!("\n=== 2. else if 多重条件 ===");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // 输出，只执行第一个为真的
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("\n=== 3. if 表达式赋值 ===");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number); // 输出: 5

    println!("\n=== 4. loop 返回值 ===");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result); // 输出: 20

    println!("\n=== 5. while 循环 ===");
    let mut number = 3;
    while number != 0 {
        print!("{}! ", number);
        number -= 1;
    }
    println!("\nLIFTOFF!!!"); // 输出: 3! 2! 1! LIFTOFF!!!

    println!("\n=== 6. for 遍历数组 ===");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        print!("{} ", element);
    }
    println!(); // 输出: 10 20 30 40 50

    println!("\n=== 7. for + Range 倒计时 ===");
    for number in (1..4).rev() {
        print!("{}! ", number);
    }
    println!("\nLIFTOFF!!!"); // 输出: 3! 2! 1! LIFTOFF!!!
}

