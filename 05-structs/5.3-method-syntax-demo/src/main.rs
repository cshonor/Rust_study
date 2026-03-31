// 5.2/5.3 使用结构体的代码例子 + 方法语法：计算长方形面积

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 与字段同名的方法：getter 风格示例
    fn width(&self) -> bool {
        self.width > 0
    }

    // 关联函数（不带 self）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("=== 版本 1：分别用变量 ===");
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area_v1(width1, height1)
    );

    println!("\n=== 版本 2：用元组 ===");
    let rect2 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_v2(rect2)
    );

    println!("\n=== 版本 3：用结构体 + 借用 ===");
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_v3(&rect3)
    );

    println!("\n=== 版本 4：方法语法（impl） ===");
    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );

    let rect_small = Rectangle {
        width: 10,
        height: 40,
    };
    let rect_big = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect3 hold rect_small? {}", rect3.can_hold(&rect_small));
    println!("Can rect3 hold rect_big? {}", rect3.can_hold(&rect_big));

    if rect3.width() {
        println!("The rectangle has a nonzero width; it is {}", rect3.width);
    }

    let sq = Rectangle::square(3);
    println!("square = {:?}, area = {}", sq, sq.area());

    println!("\n=== Debug 打印 ===");
    println!("rect3 is {:?}", rect3);
    println!("rect3 is {:#?}", rect3);

    println!("\n=== dbg! 宏（输出到 stderr） ===");
    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect4);
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

