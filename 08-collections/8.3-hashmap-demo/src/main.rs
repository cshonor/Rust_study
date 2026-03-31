// 8.3 HashMap<K, V> - 示例

use std::collections::HashMap;

fn main() {
    println!("=== 1) new + insert ===");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {:?}", scores);

    println!("\n=== 2) zip + collect ===");
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores2 = {:?}", scores2);

    println!("\n=== 3) 所有权：String 会 move 进 map ===");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map = {:?}", map);
    // field_name/field_value 此后不可用（已 move），示例保留为注释：
    // println!("{}", field_name);

    println!("\n=== 4) get 返回 Option<&V> ===");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score({}) = {:?}", team_name, score);

    println!("\n=== 5) 遍历（顺序不保证） ===");
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    println!("\n=== 6) 覆盖旧值 ===");
    scores.insert(String::from("Blue"), 25);
    println!("scores = {:?}", scores);

    println!("\n=== 7) entry / or_insert 只在不存在时插入 ===");
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores = {:?}", scores);

    println!("\n=== 8) 基于旧值更新：词频统计 ===");
    let text = "hello world wonderful world";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("counts = {:?}", counts);
}

