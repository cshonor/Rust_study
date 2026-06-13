//! RFR Chapter 02 · Layout demo
//! Run: cargo run --manifest-path 02-RFR/Chapter-02-Types/layout-demo/Cargo.toml

use std::mem::{align_of, size_of};
use std::ptr::NonNull;

#[derive(Debug)]
struct TestRust {
    a: u8,
    b: u32,
    c: u8,
}

#[repr(C)]
#[derive(Debug)]
struct TestC {
    a: u8,
    b: u32,
    c: u8,
}

#[repr(packed)]
#[derive(Debug)]
struct TestPacked {
    a: u8,
    b: u32,
    c: u8,
}

#[derive(Debug)]
struct DefaultOrder {
    a: u8,
    b: u32,
    c: u16,
}

#[repr(C)]
#[derive(Debug)]
struct DefaultOrderC {
    a: u8,
    b: u32,
    c: u16,
}

enum Either {
    A(u32),
    B(u64),
}

enum Void {}

macro_rules! show_layout {
    ($label:expr, $ty:ty) => {
        println!("--- {} ---", $label);
        println!("  size_of  = {}", size_of::<$ty>());
        println!("  align_of = {}", align_of::<$ty>());
    };
}

macro_rules! show_offsets {
    ($label:expr, $ty:ty, $($f:ident),+ $(,)?) => {
        println!("--- {} ---", $label);
        println!("  size_of  = {}", size_of::<$ty>());
        println!("  align_of = {}", align_of::<$ty>());
        $( println!("  offset {} = {}", stringify!($f), std::mem::offset_of!($ty, $f)); )+
        println!();
    };
}

fn main() {
    println!("target: {}", std::env::consts::ARCH);
    println!();

    show_offsets!(
        "repr(Rust) · Test { a: u8, b: u32, c: u8 }",
        TestRust,
        a,
        b,
        c
    );
    show_offsets!("repr(C) · Test { a: u8, b: u32, c: u8 }", TestC, a, b, c);
    show_offsets!(
        "repr(packed) · Test { a: u8, b: u32, c: u8 }",
        TestPacked,
        a,
        b,
        c
    );

    show_offsets!(
        "repr(Rust) · DefaultOrder { a: u8, b: u32, c: u16 }",
        DefaultOrder,
        a,
        b,
        c
    );
    show_offsets!(
        "repr(C) · DefaultOrder { a: u8, b: u32, c: u16 }",
        DefaultOrderC,
        a,
        b,
        c
    );

    show_layout!("enum Either { A(u32), B(u64) }", Either);
    show_layout!("enum Void {}", Void);

    println!("--- Niche optimization ---");
    println!("  size_of::<&i32>()         = {}", size_of::<&i32>());
    println!(
        "  size_of::<Option<&i32>>() = {}",
        size_of::<Option<&i32>>()
    );
    println!(
        "  size_of::<NonNull<i32>>() = {}",
        size_of::<NonNull<i32>>()
    );
    println!(
        "  size_of::<Option<NonNull<i32>>>() = {}",
        size_of::<Option<NonNull<i32>>>()
    );
    println!();

    // --- raw bytes demo (repr(C) only) ---
    #[repr(C)]
    struct MyStruct {
        a: u8,
        b: u32,
        c: u16,
    }

    println!("--- repr(C) MyStruct + raw bytes ---");
    println!("  size_of  = {}", size_of::<MyStruct>());
    println!("  align_of = {}", align_of::<MyStruct>());
    println!("  a @ {}", std::mem::offset_of!(MyStruct, a));
    println!("  b @ {}", std::mem::offset_of!(MyStruct, b));
    println!("  c @ {}", std::mem::offset_of!(MyStruct, c));

    let s = MyStruct {
        a: 0x12,
        b: 0x5678_abcd,
        c: 0xef90,
    };
    dump_bytes("MyStruct", &s);
}

fn dump_bytes<T>(label: &str, val: &T) {
    let n = size_of::<T>();
    let ptr = val as *const T as *const u8;
    let bytes = unsafe { std::slice::from_raw_parts(ptr, n) };
    println!("  {label} raw ({n} B): {:02x?}", bytes);
    println!("  (padding slots may show stack garbage; use offset_of for layout)");
}
