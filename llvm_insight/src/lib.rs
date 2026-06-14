//! 统一 IR 实验入口：从仓库根执行
//! `cargo rustc --manifest-path llvm_insight/Cargo.toml -p llvm_insight_lab -- --emit=llvm-ir`
//! （若在 workspace 中已加入本目录，则可用 `-p llvm_insight_lab`。）
//!
//! 精读第 4、7 章时在此增删函数；归档时将 `.ll` 复制到 `ir_samples/` 对应子目录。

use std::sync::atomic::{fence, AtomicU64, Ordering};

// --- ch04: 静态分发 (monomorph) vs 动态分发 (dyn / vtable) ---

pub trait TickHandler {
    fn on_tick(&self) -> u64;
}

pub struct HandlerA(pub u64);
pub struct HandlerB(pub u64);

impl TickHandler for HandlerA {
    fn on_tick(&self) -> u64 {
        self.0.wrapping_add(1)
    }
}

impl TickHandler for HandlerB {
    fn on_tick(&self) -> u64 {
        self.0.wrapping_mul(2)
    }
}

/// 单态化：编译期为每个 `T` 生成专属 `process_static::<T>`
#[inline(never)]
pub fn process_static<T: TickHandler>(h: &T) -> u64 {
    h.on_tick()
}

/// 动态分发：经 vtable 间接调用
#[inline(never)]
pub fn process_dyn(h: &dyn TickHandler) -> u64 {
    h.on_tick()
}

#[inline(never)]
pub fn demo_static_dispatch() -> u64 {
    let a = HandlerA(10);
    let b = HandlerB(20);
    process_static(&a).wrapping_add(process_static(&b))
}

#[inline(never)]
pub fn demo_dyn_dispatch() -> u64 {
    let a = HandlerA(10);
    let b = HandlerB(20);
    let ha: &dyn TickHandler = &a;
    let hb: &dyn TickHandler = &b;
    process_dyn(ha).wrapping_add(process_dyn(hb))
}

// --- ch04/ch07: 原子与内存序 ---

#[inline]
pub fn load_relaxed(counter: &AtomicU64) -> u64 {
    counter.load(Ordering::Relaxed)
}

#[inline]
pub fn load_seqcst(counter: &AtomicU64) -> u64 {
    counter.load(Ordering::SeqCst)
}

#[inline]
pub fn store_release(counter: &AtomicU64, v: u64) {
    counter.store(v, Ordering::Release);
}

#[inline]
pub fn fence_seqcst() {
    fence(Ordering::SeqCst);
}

#[inline]
pub fn add_plain(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
