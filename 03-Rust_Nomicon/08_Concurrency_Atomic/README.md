# 08 · Concurrency and Parallelism

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] 已读（笔记整理）
- [x] 示例 crate（data race 边界 / Send·Sync / atomics）

---

## 一句话

**并发章** — Safe 消除数据竞争但不防死锁；Send/Sync 标记；C++20 内存序与 Relaxed / Acquire-Release / SeqCst。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位与要点 | [00-overview.md](./00-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/data_races.rs](./src/data_races.rs) | 安全访问 vs TOCTOU + unsafe 风险 |
| [src/send_sync.rs](./src/send_sync.rs) | `Send`/`Sync` 编译期边界 |
| [src/atomics.rs](./src/atomics.rs) | Relaxed 计数、Release/Acquire、SeqCst |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/08_Concurrency_Atomic
cargo run
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| Send/Sync 入门 | [Book 16.4 demo](../../00-Book/16-fearless-concurrency/16.4-send-sync-demo/) |
| 内部可变性 | [RFR 07-interior-mutability](../../02-RFR/Chapter-01-Foundations/07-interior-mutability.md) |
| 上一章 | [07_Panic_Safety](../07_Panic_Safety/README.md) |
| 下一章 | [09_Impl_Vec_Arc](../09_Impl_Vec_Arc/README.md) · 实现 Arc |

---

## 逻辑脉络

数据竞争边界 → Send/Sync → 原子与内存序 → 进入 Implementing Vec/Arc。
