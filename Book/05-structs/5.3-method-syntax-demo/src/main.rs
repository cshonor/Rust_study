//! 5.3 方法语法：三种 `self` + 关联函数

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// 1. `&self`：不可变借用，只读
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 2. `&mut self`：可变借用，可改字段（调用方变量须 `mut`）
    fn set_size(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }

    /// 3. `self`：消费实例，所有权移入方法
    fn destroy_to_tuple(self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// 关联函数：无 `self`，`Rectangle::new()` 调用
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("=== 三种 self + 关联函数（核心） ===");
    let mut rect = Rectangle::new(10, 20);

    // &self：只读，rect 仍可用
    let s = rect.area();
    println!("面积：{s}");

    // &mut self：修字段，rect 须 mut
    rect.set_size(50, 60);
    println!("修改后 {rect:?}");

    // self：move 进方法，此后 rect 不可用
    let (w, h) = rect.destroy_to_tuple();
    println!("拆分数据：{w} {h}");
    // println!("{:?}", rect); // ❌ value moved

    println!("\n=== 5.2 演进：独立函数 → 方法 ===");
    demo_area_evolution();

    println!("\n=== 其它：can_hold / square / dbg! ===");
    let rect3 = Rectangle::new(30, 50);
    let small = Rectangle::new(10, 40);
    let big = Rectangle::new(60, 45);
    println!("can_hold small? {}", rect3.can_hold(&small));
    println!("can_hold big? {}", rect3.can_hold(&big));
    if rect3.width() {
        println!("rect3.width field = {}", rect3.width);
    }
    let sq = Rectangle::square(3);
    println!("square = {sq:?}, area = {}", sq.area());
    dbg!(&sq);
}

fn demo_area_evolution() {
    let w = 30;
    let h = 50;
    println!("v1 变量: {}", area_v1(w, h));
    println!("v2 元组: {}", area_v2((w, h)));
    let r = Rectangle::new(w, h);
    println!("v3 &结构体: {}", area_v3(&r));
    println!("v4 方法: {}", r.area());
}

fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
