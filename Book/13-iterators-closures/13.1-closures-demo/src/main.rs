use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout_without_cache(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_closure(intensity));
    }
}

struct CacherOnce<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> CacherOnce<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct CacherMap<F, A, R>
where
    F: Fn(&A) -> R,
    A: Eq + Hash,
    R: Clone,
{
    calculation: F,
    values: HashMap<A, R>,
}

impl<F, A, R> CacherMap<F, A, R>
where
    F: Fn(&A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    fn new(calculation: F) -> Self {
        Self {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> R {
        if let Some(v) = self.values.get(&arg) {
            return v.clone();
        }
        let v = (self.calculation)(&arg);
        self.values.insert(arg, v.clone());
        v
    }
}

fn generate_workout_with_cache(intensity: u32, random_number: u32) {
    let mut expensive = CacherOnce::new(simulated_expensive_calculation);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive.value(intensity));
        println!("Next, do {} situps!", expensive.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive.value(intensity));
    }
}

fn closure_capture_examples() {
    // Fn：不可变借用捕获
    let x = 4;
    let equal_to_x = |z: i32| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // FnMut：可变借用捕获
    let mut count = 0;
    let mut inc = || {
        count += 1;
        count
    };
    assert_eq!(1, inc());
    assert_eq!(2, inc());

    // FnOnce：move 捕获并消费
    let v = vec![1, 2, 3];
    let consume = move || v;
    let got = consume();
    assert_eq!(got, vec![1, 2, 3]);
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    println!("== Without cache ==");
    generate_workout_without_cache(simulated_user_specified_value, simulated_random_number);

    println!("\n== With cache (CacherOnce) ==");
    generate_workout_with_cache(simulated_user_specified_value, simulated_random_number);

    // 通用缓存（HashMap）演示：同一个闭包对不同输入各缓存一次
    let mut c = CacherMap::new(|a: &u32| -> u32 { *a });
    assert_eq!(1, c.value(1));
    assert_eq!(2, c.value(2));
    assert_eq!(1, c.value(1));

    closure_capture_examples();
    println!("\nAll closure examples passed.");
}

