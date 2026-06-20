//! RFR ch03 · 01-2 as_ / into_ / get_ / try_ 四系列可运行示例
//!
//! ```bash
//! cargo run --manifest-path 02-RFR/Chapter-03-Designing-Interfaces/naming-series-demo/Cargo.toml
//! ```

use std::sync::Mutex;

fn main() {
    demo_as();
    demo_into();
    demo_get();
    demo_try();
    println!("all naming-series demos ok");
}

fn demo_as() {
    let s = String::from("hello");
    let s1 = s.as_str();
    let s2 = s.as_str();
    assert_eq!(s, "hello");
    assert_eq!(s1, "hello");
    assert_eq!(s2, "hello");

    let x = Some(10);
    let r1 = x.as_ref();
    let r2 = x.as_ref();
    assert!(x.is_some());
    assert_eq!(r1, r2);
}

fn demo_into() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.into_iter();
    // println!("{:?}", v); // compile error: v moved
    assert_eq!(v_iter.next(), Some(1));
    assert_eq!(v_iter.next(), Some(2));
    assert_eq!(v_iter.next(), Some(3));
    assert_eq!(v_iter.next(), None);
}

fn demo_get() {
    let v = vec![1, 2, 3];
    assert_eq!(v.get(1), Some(&2));
    assert_eq!(v.get(5), None);
    // let _bad = v[5]; // panic

    let mut w = vec![10, 20, 30];
    if let Some(x) = w.get_mut(0) {
        *x += 1;
    }
    assert_eq!(w, vec![11, 20, 30]);
}

fn demo_try() {
    let lock = Mutex::new(5);
    let mut guard = lock.lock().unwrap();
    *guard = 10;
    assert!(lock.try_lock().is_err());
    drop(guard);

    let guard2 = lock.try_lock().expect("lock should be free");
    assert_eq!(*guard2, 10);
}
