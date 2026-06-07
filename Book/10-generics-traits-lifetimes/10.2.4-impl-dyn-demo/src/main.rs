// 10.2.4 impl vs dyn · 10.2.6 胖指针与虚表 demo
//   cargo run           — impl vs dyn 全段
//   cargo run -- multi_vtable — 10.2.7 多 Trait 多虚表

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

// ── 10.2.7：A 实现 T1/T2/T3 → 3 张虚表 ───────────────
trait T1 {
    fn f1(&self);
}
trait T2 {
    fn f2(&self);
}
trait T3 {
    fn f3(&self);
}

struct A;

impl T1 for A {
    fn f1(&self) {
        println!("  T1::f1");
    }
}
impl T2 for A {
    fn f2(&self) {
        println!("  T2::f2");
    }
}
impl T3 for A {
    fn f3(&self) {
        println!("  T3::f3");
    }
}

fn use_t1(x: &dyn T1) {
    x.f1();
}
fn use_t2(x: &dyn T2) {
    x.f2();
}
fn use_t3(x: &dyn T3) {
    x.f3();
}

fn demo_multi_vtable() {
    println!("=== 10.2.7 A 实现 T1+T2+T3 → 3 张独立虚表 ===");
    let a = A;

    println!("\n  dyn T1（vtable → A 的 T1 表）:");
    use_t1(&a);
    println!("  dyn T2（vtable → A 的 T2 表）:");
    use_t2(&a);
    println!("  dyn T3（vtable → A 的 T3 表）:");
    use_t3(&a);

    println!("\n  同一 A：分别 &dyn T1 与 &dyn T2（各查各虚表，无合并表）:");
    use_t1(&a);
    use_t2(&a);
    // fn bad(x: &dyn T1 + T2) {} // ❌ stable：E0225 两个 non-auto trait

    println!("\nok: multi_vtable demo 完成");
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

    if mode == "multi_vtable" {
        demo_multi_vtable();
        return;
    }

    run_full();
}
