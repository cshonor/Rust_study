use std::fmt;
use std::ops::Add;

// ----------------------------
// 1) 关联类型（Associated Types）
// ----------------------------

trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug)]
struct Counter {
    cur: u32,
    end: u32,
}

impl Counter {
    fn new(end: u32) -> Self {
        Self { cur: 0, end }
    }
}

impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            None
        } else {
            self.cur += 1;
            Some(self.cur)
        }
    }
}

// -----------------------------------------
// 2) 默认泛型参数 + 运算符重载（std::ops::Add）
// -----------------------------------------

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Millimeters(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

// ------------------------------------
// 3) 完全限定语法：同名方法/关联函数
// ------------------------------------

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        "Spot".to_string()
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        "puppy".to_string()
    }
}

// ------------------------
// 4) 父 trait（supertrait）
// ------------------------

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.chars().count();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// ---------------------------------------
// 5) newtype：为外部类型实现外部 trait
// ---------------------------------------

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("--- associated types ---");
    let mut c = Counter::new(3);
    while let Some(v) = c.next() {
        println!("Counter next: {v}");
    }

    println!("--- operator overloading (Add) ---");
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    println!("Point add ok.");

    assert_eq!(Millimeters(500) + Meters(2), Millimeters(2500));
    println!("Millimeters + Meters ok.");

    println!("--- disambiguation (same method name) ---");
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("--- fully qualified syntax (associated function) ---");
    println!("Dog::baby_name() = {}", Dog::baby_name());
    println!(
        "<Dog as Animal>::baby_name() = {}",
        <Dog as Animal>::baby_name()
    );

    println!("--- supertrait (OutlinePrint: Display) ---");
    let p = Point { x: 1, y: 3 };
    p.outline_print();

    println!("--- newtype pattern ---");
    let w = Wrapper(vec!["hello".into(), "world".into()]);
    println!("w = {w}");
}

