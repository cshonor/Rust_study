//! RFR Chapter 02 · Layout demo
//! Run: cargo run --manifest-path 02-RFR/Chapter-02-Types/layout-demo/Cargo.toml

use std::mem::{align_of, offset_of, size_of};

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

macro_rules! show_layout {
    ($label:expr, $ty:ty) => {
        println!("--- {} ---", $label);
        println!("  size_of  = {}", size_of::<$ty>());
        println!("  align_of = {}", align_of::<$ty>());
        println!("  offset a = {}", offset_of!($ty, a));
        println!("  offset b = {}", offset_of!($ty, b));
        println!("  offset c = {}", offset_of!($ty, c));
        println!();
    };
}

fn main() {
    println!("target: {}", std::env::consts::ARCH);
    println!();

    show_layout!(
        "repr(Rust) default · Test { a: u8, b: u32, c: u8 }",
        TestRust
    );
    show_layout!("repr(C) · Test { a: u8, b: u32, c: u8 }", TestC);
    show_layout!(
        "repr(packed) · Test { a: u8, b: u32, c: u8 }",
        TestPacked
    );
}
