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

// ── §1 入参 impl（静态）──────────────────────────────
fn speak_impl(animal: impl Animal) {
    animal.cry();
}

// ── §2 入参 &dyn（动态）──────────────────────────────
fn speak_dyn(animal: &dyn Animal) {
    animal.cry();
}

// ── §3 返回 impl（固定 Dog）──────────────────────────
fn get_animal_impl() -> impl Animal {
    Dog
}

// ── §4 返回 Box<dyn>（分支 Dog/Cat）──────────────────
fn get_animal_dyn(flag: bool) -> Box<dyn Animal> {
    if flag {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

// ── bonus：闭包返回 impl Fn ───────────────────────────
fn make_adder() -> impl Fn(i32) -> i32 {
    |x| x + 5
}

fn main() {
    println!("=== §1 speak_impl(Dog) / speak_impl(Cat) — 静态单态化 ===");
    speak_impl(Dog);
    speak_impl(Cat);

    println!("\n=== §2 speak_dyn + Vec<&dyn Animal> — 异构集合 ===");
    let dog = Dog;
    let cat = Cat;
    speak_dyn(&dog);
    speak_dyn(&cat);
    let list: Vec<&dyn Animal> = vec![&dog, &cat];
    for item in list {
        item.cry();
    }

    println!("\n=== §3 get_animal_impl() -> impl Animal（固定 Dog）===");
    get_animal_impl().cry();
    // fn bad(flag: bool) -> impl Animal { if flag { Dog } else { Cat } } // ❌

    println!("\n=== §4 get_animal_dyn(flag) -> Box<dyn Animal> ===");
    get_animal_dyn(true).cry();
    get_animal_dyn(false).cry();

    println!("\n=== bonus make_adder() -> impl Fn ===");
    let add5 = make_adder();
    println!("  add5(10) = {}", add5(10));

    println!("\nok: impl vs dyn demo 完成");
}
