use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn fast_calc(intensity: u32) -> u32 {
    intensity
}

fn generate_workout_without_cache(intensity: u32, random_number: u32, calc: fn(u32) -> u32) {
    if intensity < 25 {
        println!("Today, do {} pushups!", calc(intensity));
        println!("Next, do {} situps!", calc(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", calc(intensity));
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

fn generate_workout_with_cache(intensity: u32, random_number: u32, calc: fn(u32) -> u32) {
    let mut expensive = CacherOnce::new(calc);

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
    let x = 4;
    let equal_to_x = |z: i32| z == x;
    assert!(equal_to_x(4));

    let mut count = 0;
    let mut inc = || {
        count += 1;
        count
    };
    assert_eq!(1, inc());
    assert_eq!(2, inc());

    let v = vec![1, 2, 3];
    let consume = move || v;
    assert_eq!(consume(), vec![1, 2, 3]);
}

fn thread_move_example() {
    let s = String::from("hello");
    let handle = thread::spawn(move || {
        println!("thread move: {s}");
    });
    handle.join().unwrap();
}

fn run_quick() {
    println!("== quick: CacherOnce (fast_calc) ==");
    generate_workout_with_cache(10, 7, fast_calc);

    let mut c = CacherMap::new(|a: &u32| -> u32 { a * 10 });
    assert_eq!(10, c.value(1));
    assert_eq!(20, c.value(2));
    assert_eq!(10, c.value(1));

    closure_capture_examples();
    thread_move_example();
    println!("quick: all passed");
}

fn run_full() {
    println!("== Without cache (slow x2) ==");
    generate_workout_without_cache(10, 7, simulated_expensive_calculation);

    println!("\n== With CacherOnce (slow x1) ==");
    generate_workout_with_cache(10, 7, simulated_expensive_calculation);

    let mut c = CacherMap::new(|a: &u32| -> u32 { *a });
    assert_eq!(1, c.value(1));
    assert_eq!(2, c.value(2));

    closure_capture_examples();
    thread_move_example();
    println!("\nAll closure examples passed.");
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "full".to_string());
    match mode.as_str() {
        "quick" => run_quick(),
        "full" | _ => run_full(),
    }
}
