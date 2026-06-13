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
}
