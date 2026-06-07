// 10.2.4 impl vs dyn · 10.2.6 胖指针与虚表 demo
//   cargo run           — impl vs dyn 全段
//   cargo run -- vtable — 胖指针 size · 多 trait 虚表

trait Animal {
    fn cry(&self);
}

trait Run {
    fn run(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn cry(&self) {
        println!("  汪汪");
    }
}

impl Animal for Cat {
    fn cry(&self) {
        println!("  喵喵");
    }
}

impl Run for Dog {
    fn run(&self) {
        println!("  跑");
    }
}

fn speak_impl(animal: impl Animal) {
    animal.cry();
}

fn speak_dyn(animal: &dyn Animal) {
    animal.cry();
}

fn get_animal_impl() -> impl Animal {
    Dog
}

fn get_animal_dyn(flag: bool) -> Box<dyn Animal> {
    if flag {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

fn make_adder() -> impl Fn(i32) -> i32 {
    |x| x + 5
}

fn demo_vtable() {
    println!("=== 10.2.6 胖指针 = 数据 ptr + vtable ptr（同时存好）===");

    let dog = Dog;
    let p_animal: &dyn Animal = &dog;
    let p_run: &dyn Run = &dog;

    println!("  size_of &Dog          = {} B（1 个指针）", std::mem::size_of::<&Dog>());
    println!(
        "  size_of &dyn Animal   = {} B（2 个指针：数据 + vtable）",
        std::mem::size_of::<&dyn Animal>()
    );
    println!(
        "  size_of &dyn Run      = {} B（换 trait = 换 vtable 字段）",
        std::mem::size_of::<&dyn Run>()
    );

    println!("\n  同一 dog，不同 dyn trait 调用：");
    p_animal.cry(); // vtable → Animal::cry，数据 ptr → dog
    p_run.run(); // vtable → Run::run，数据 ptr → 仍是 dog

    let dog2 = Dog;
    let p2: &dyn Animal = &dog2;
    println!("\n  两个 Dog 实例 → 共用 Dog 类型的 Animal 虚表（类型级，非实例级）");
    p2.cry();

    println!("\nok: vtable 胖指针 demo 完成");
}

fn run_full() {
    println!("=== §1 speak_impl — 静态单态化 ===");
    speak_impl(Dog);
    speak_impl(Cat);

    println!("\n=== §2 Vec<&dyn Animal> — 借用异构 ===");
    let dog = Dog;
    let cat = Cat;
    speak_dyn(&dog);
    speak_dyn(&cat);
    for item in [&dog as &dyn Animal, &cat] {
        item.cry();
    }

    println!("\n=== §2b Vec<Box<dyn Animal>> — 所有权异构 ===");
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    for a in animals {
        a.cry();
    }

    println!("\n=== §3 -> impl Animal（固定 Dog）===");
    get_animal_impl().cry();

    println!("\n=== §4 -> Box<dyn Animal>（分支 Dog/Cat）===");
    get_animal_dyn(true).cry();
    get_animal_dyn(false).cry();

    println!("\n=== bonus -> impl Fn ===");
    println!("  add5(10) = {}", make_adder()(10));

    println!("\nok: impl vs dyn demo 完成（-- vtable 看胖指针）");
}

fn main() {
    let arg = std::env::args().nth(1);
    let mode = arg.as_deref().unwrap_or("full");

    if mode == "vtable" {
        demo_vtable();
        return;
    }

    run_full();
}
