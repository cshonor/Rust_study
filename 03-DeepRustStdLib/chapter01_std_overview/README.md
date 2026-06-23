# 第 1 章 · Rust 标准库体系概述

> 所属：[03 DeepRustStdLib](../README.md) · 后：[第 2 章 Rust 特性小结](../chapter02_rust_features_summary/README.md)

**本章目标**：在拆具体模块 / 读 `libstd` 源码之前，建立 **`core` → `alloc` → `std`** 分层、核心模块地图、设计哲学与读源码方法。

---

## 子节索引

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **1.1** | 标准库定位：`core` / `alloc` / `std` 分层 | [1.1-stdlib-positioning.md](./1.1-stdlib-positioning.md) |
| **1.2** | 核心模块概览：集合、并发、IO、FFI 边界 | [1.2-core-modules-overview.md](./1.2-core-modules-overview.md) |
| **1.3** | 设计哲学：零成本抽象、内存安全、封装性 | [1.3-design-philosophy.md](./1.3-design-philosophy.md) |
| **1.4** | 如何阅读标准库源码：工具、路径、方法 | [1.4-reading-stdlib-source.md](./1.4-reading-stdlib-source.md) |

**阅读顺序**：**1.1 → 1.2 → 1.3 → 1.4**

---

## 与主线对照

| 本章概念 | 本仓库延伸 |
|----------|------------|
| `core` / 无堆 | [RFR Ch12 no_std](../../02-RFR/Chapter-12-Rust-Without-Standard-Library/README.md) · [Nomicon 10_NoStd](../../04-Rust-Nomicon/10_NoStd/README.md) |
| 零成本抽象 | [RFR Ch02 类型](../../02-RFR/Chapter-02-Types/README.md) · [Book 10 trait](../../00-Book/10-generics-traits-lifetimes/) |
| `unsafe` 在 std 边界 | [Nomicon 01](../../04-Rust-Nomicon/01_Safe_Unsafe/README.md) · [RFR Ch09](../../02-RFR/Chapter-09-Unsafe-Code/README.md) |
| 并发 / IO 模块 | [05-atomic](../../05-Async-Concurrency-Network/01-atomic/README-学习区.md) · [05-rust_network](../../05-Async-Concurrency-Network/03-rust_network_programming/README.md) |
