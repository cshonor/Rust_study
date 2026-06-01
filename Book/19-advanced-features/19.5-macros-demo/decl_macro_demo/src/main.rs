// 精简版 vec!：接受 0..n 个表达式，展开为创建 Vec 并 push。
macro_rules! my_vec {
    () => {{
        Vec::new()
    }};
    ( $( $x:expr ),+ $(,)? ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )+
        temp_vec
    }};
}

fn main() {
    println!("--- macro_rules! demo ---");
    let v1: Vec<u32> = my_vec![1, 2, 3];
    let v2: Vec<&str> = my_vec!["a", "b", "c",];
    let v3: Vec<i32> = my_vec![];

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v3 = {:?}", v3);
}

