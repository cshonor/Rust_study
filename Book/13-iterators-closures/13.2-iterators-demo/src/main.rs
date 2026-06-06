// 13.2 迭代器 demo — cargo run 逐段演示（对应 13.2.1）

use iterators_demo::{get_dyn, get_iter, lazy_map_filter, Counter, CounterRange};

fn main() {
    println!("=== §1 惰性：collect 前 map/filter 不执行 ===");
    let res = lazy_map_filter([1, 2, 3]);
    println!("  collect 后: {:?}", res);

    println!("\n=== §2 手动 next（须 mut）===");
    let arr = [1, 2, 3];
    let mut iter = arr.iter();
    println!("  next → {:?}, {:?}, {:?}", iter.next(), iter.next(), iter.next());
    println!("  结束 → {:?}", iter.next());

    println!("\n=== §3 iter / iter_mut / into_iter ===");
    let mut v = vec![10, 20, 30];
    print!("  iter 只读: ");
    for x in v.iter() {
        print!("{} ", x);
    }
    println!();
    for x in v.iter_mut() {
        *x += 1;
    }
    println!("  iter_mut 后: {:?}", v);
    let moved: Vec<i32> = v.into_iter().collect();
    println!("  into_iter 拿走: {:?}", moved);

    println!("\n=== §4 impl Iterator vs Box<dyn Iterator> ===");
    let a: Vec<_> = get_iter().collect();
    println!("  get_iter (impl): {:?}", a);
    println!("  get_dyn(true):  {:?}", get_dyn(true).collect::<Vec<_>>());
    println!("  get_dyn(false): {:?}", get_dyn(false).collect::<Vec<_>>());

    println!("\n=== §5 CounterRange + Counter 链式 ===");
    let mut cr = CounterRange { curr: 0, max: 3 };
    print!("  CounterRange 0..3: ");
    while let Some(n) = cr.next() {
        print!("{} ", n);
    }
    println!();
    let sum: usize = CounterRange { curr: 1, max: 5 }
        .map(|x| x * 2)
        .sum();
    println!("  CounterRange map*2 sum(1..4): {}", sum);

    let zip_sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("  Counter zip/skip/filter sum: {}", zip_sum);

    println!("\nok: 迭代器 demo 完成");
}
