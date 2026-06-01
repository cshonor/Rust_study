use std::collections::HashMap;
use std::mem;

// ----------------------------
// 1) newtype：类型安全 + 抽象
// ----------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Millimeters(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Meters(u32);

fn takes_mm(x: Millimeters) -> u32 {
    x.0
}

#[derive(Debug, Default)]
struct People(HashMap<i32, String>);

impl People {
    fn add(&mut self, id: i32, name: impl Into<String>) {
        self.0.insert(id, name.into());
    }

    fn name_of(&self, id: i32) -> Option<&str> {
        self.0.get(&id).map(|s| s.as_str())
    }
}

// -----------------------
// 2) type alias：同义词
// -----------------------

type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_thunk(t: Thunk) {
    t();
}

fn returns_thunk(msg: &'static str) -> Thunk {
    Box::new(move || println!("{msg}"))
}

type IoResult<T> = std::result::Result<T, std::io::Error>;

fn io_style_ok() -> IoResult<()> {
    Ok(())
}

// -------------------------
// 3) never type：`!` 的参与
// -------------------------

fn diverge_with_panic() -> ! {
    panic!("diverging function: never returns")
}

fn parse_or_continue(inputs: &[&str]) -> u32 {
    let mut last = 0u32;
    for s in inputs {
        let v: u32 = match s.parse::<u32>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        last = v;
    }
    last
}

fn match_with_panic_branch(x: Result<u32, &'static str>) -> u32 {
    match x {
        Ok(v) => v,
        Err(e) => {
            let _ = e;
            diverge_with_panic()
        }
    }
}

// ------------------------------------
// 4) DST / Sized / ?Sized：放在指针后面
// ------------------------------------

trait Greeter {
    fn greet(&self) -> String;
}

struct Robot;

impl Greeter for Robot {
    fn greet(&self) -> String {
        "beep".to_string()
    }
}

fn accepts_unsized<T: ?Sized>(v: &T) -> usize {
    let _ = v;
    mem::size_of::<&T>()
}

fn main() {
    println!("--- newtype: type safety + abstraction ---");
    let mm = Millimeters(1200);
    let _m = Meters(2);
    println!("takes_mm(Millimeters(1200)) = {}", takes_mm(mm));

    let mut people = People::default();
    people.add(1, "Alice");
    people.add(2, "Bob");
    println!("people.name_of(2) = {:?}", people.name_of(2));

    println!("--- type alias ---");
    let x: i32 = 5;
    let y: Kilometers = 7;
    println!("x + y (i32 + Kilometers) = {}", x + y);

    takes_thunk(returns_thunk("Thunk called."));
    println!("io_style_ok() is_ok = {}", io_style_ok().is_ok());

    println!("--- never type `!` ---");
    let last = parse_or_continue(&["10", "nope", "20"]);
    println!("parse_or_continue last = {last}");
    println!("match_with_panic_branch(Ok(1)) = {}", match_with_panic_branch(Ok(1)));

    println!("--- DST / Sized / ?Sized ---");
    let s: &str = "hello";
    println!("size_of::<&str>() = {}", mem::size_of::<&str>());
    println!("accepts_unsized::<str>(&str) returns {}", accepts_unsized::<str>(s));

    let r = Robot;
    let g: &dyn Greeter = &r;
    println!("g.greet() = {}", g.greet());
    println!("size_of::<&dyn Greeter>() = {}", mem::size_of::<&dyn Greeter>());
    println!(
        "accepts_unsized::<dyn Greeter>(&dyn) returns {}",
        accepts_unsized::<dyn Greeter>(g)
    );
}

