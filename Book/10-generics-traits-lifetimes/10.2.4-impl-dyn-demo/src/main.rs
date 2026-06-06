// 10.2.4 impl Trait vs dyn Trait — Animal 对比 demo

trait Animal {
    fn cry(&self);
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

fn main() {
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

    println!("\n=== §2b Vec<Box<dyn Animal>> — 所有权异构（运行时 vtable）===");
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

    println!("\nok: impl vs dyn demo 完成");
}
