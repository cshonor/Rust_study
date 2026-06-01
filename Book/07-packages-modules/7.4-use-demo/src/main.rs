use std::{cmp::Ordering, io};
use std::io::{self as std_io, Write};

use use_demo::results;
use use_demo::results::beta::Result as BetaResult;

fn main() {
    // 1) `pub use` 重导出后的路径
    use_demo::eat_at_restaurant();
    use_demo::hosting::add_to_waitlist();

    // 2) 惯用导入：函数通常保留父模块前缀
    let ordering = Ordering::Equal;
    println!("ordering = {ordering:?}");

    // 3) 嵌套路径 + self
    let mut out = Vec::<u8>::new();
    write!(&mut out, "hello").unwrap();
    std_io::stdout().write_all(b"").unwrap(); // 只是为了真正用到 std_io
    let _ = io::stdin(); // 只是为了真正用到 io

    // 4) 同名冲突：用父模块或 as 起别名
    let a: results::alpha::Result = Ok("alpha ok");
    let b: BetaResult = Ok(42);
    println!("alpha = {a:?}, beta = {b:?}");

    // 5) glob：一次性导入所有公有项（示例用法）
    {
        use std::collections::*;
        let mut map = HashMap::new();
        map.insert(1, 2);
        println!("map len = {}", map.len());
    }
}

