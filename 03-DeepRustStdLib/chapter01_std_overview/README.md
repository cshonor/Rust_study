# 第 1 章 · Rust 标准库体系概述

> 所属：[03 DeepRustStdLib](../README.md) · 后：[第 2 章 Rust 特征小议](../chapter02_rust_features_summary/README.md)

**本章目标**：建立 **`core` → `alloc` → `std`** 三层架构 — 弄清每一层 **能做什么、不能做什么、源码在哪**（对应原书第 1 章，见 [本书目录 § 第 1 章](../本书目录.md#第-1-章--rust-标准库体系概述p1)）。

---

## 子节索引

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **1.1** | **`core` 库**：`no_std` 最小子集、语言原语与算法 | [1.1-core-crate.md](./1.1-core-crate.md) |
| **1.2** | **`alloc` 库**：堆分配、`Vec` / `String` / `Arc` 等 | [1.2-alloc-crate.md](./1.2-alloc-crate.md) |
| **1.3** | **`std` 库**：OS 抽象、重导出与运行时 glue | [1.3-std-crate.md](./1.3-std-crate.md) |
| **1.4** | **回顾**：三层依赖与适用场景 | [1.4-recap.md](./1.4-recap.md) |

**阅读顺序**：**1.1 → 1.2 → 1.3 → 1.4**（自底向上）

---

## 与主线对照

| 本章概念 | 本仓库延伸 |
|----------|------------|
| `core` / 无堆 | [RFR Ch12 no_std](../../02-RFR/Chapter-12-Rust-Without-Standard-Library/README.md) · [Nomicon 10_NoStd](../../04-Rust-Nomicon/10_NoStd/README.md) |
| `alloc` / 自定义分配器 | [Nomicon 08 Vec](../../04-Rust-Nomicon/08_Impl_Vec_Arc/README.md) |
| `std` 并发 / IO | [05-atomic](../../05-Async-Concurrency-Network/01-atomic/README-学习区.md) · [05-rust_network](../../05-Async-Concurrency-Network/03-rust_network_programming/README.md) |
| 读源码方法、设计哲学 | 见 [第 2 章附录](../chapter02_rust_features_summary/README.md#附录)（与「特性小结」配套，不属本章三节） |
