struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/// 示例 15-14：作用域结束时自动 `drop`，顺序与创建相反（先 `d` 后 `c`）。
#[allow(unused_variables)]
fn demo_drop_order() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn main() {
    demo_drop_order();

    println!("\n--- early drop (示例 15-16) ---\n");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
