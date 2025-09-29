use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("猜字谜");
    println!("===================");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("本次随机数是：{}", secret_number);
    loop {
       println!("请输入你的猜测：");

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("读取失败");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("请输入一个有效的数字");
            continue;
        }
    };
        println!("你猜的数字是：{}", guess);

    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("你猜的数字在秘密数字之前"),
        std::cmp::Ordering::Greater => println!("你猜的数字在秘密数字之后"),
        std::cmp::Ordering::Equal => println!("恭喜你，猜对了！"),
    }
    }
   
}
