# 第 2 章 · Rust 特性小结

> 所属：[03 DeepRustStdLib](../README.md) · 前：[第 1 章 体系概述](../chapter01_std_overview/README.md) · 后：[第 3 章 容器（规划）](../README.md#目录)

**本章定位**：衔接 **[RFR](../../02-RFR/RFR-本书目录.md) 内功** 与 **标准库实现** — 把所有权、trait、生命周期、闭包、模式匹配、错误、泛型从「语法点」串成 **读 `libstd` 时能认出的实现逻辑**，为后面智能指针、Unsafe 专题铺垫。

---

## 子节索引

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **2.1** | 核心特性与 `std` 结合：所有权、借用 → 库实现 | [2.1-core-features-and-std.md](./2.1-core-features-and-std.md) |
| **2.2** | Trait 系统在标准库中的落地 | [2.2-traits-in-stdlib.md](./2.2-traits-in-stdlib.md) |
| **2.3** | 生命周期在标准库中的落地 | [2.3-lifetimes-in-stdlib.md](./2.3-lifetimes-in-stdlib.md) |
| **2.4** | 闭包与 `Iterator`：迭代器设计的 trait 支撑 | [2.4-closures-iterator-in-stdlib.md](./2.4-closures-iterator-in-stdlib.md) |
| **2.5** | 模式匹配在 `std` 源码中的常用场景 | [2.5-pattern-matching-in-stdlib.md](./2.5-pattern-matching-in-stdlib.md) |
| **2.6** | 错误处理的标准库规范 | [2.6-error-handling-in-stdlib.md](./2.6-error-handling-in-stdlib.md) |
| **2.7** | 泛型编程的标准库规范 | [2.7-generics-in-stdlib.md](./2.7-generics-in-stdlib.md) |

**阅读顺序**：**2.1 → 2.2 → … → 2.7**（2.2～2.4 可对照 RFR 第 2 章反复跳读）

---

## 与 RFR / Book 对照

| 本章 | RFR | The Book |
|------|-----|----------|
| 2.1 所有权 | [Ch01 Foundations](../../02-RFR/Chapter-01-Foundations/README.md) | [04 所有权](../../00-Book/04-ownership/) |
| 2.2 Trait | [Ch02 Types · trait](../../02-RFR/Chapter-02-Types/08-trait-bounds.md) | [10 泛型与 trait](../../00-Book/10-generics-traits-lifetimes/) |
| 2.3 生命周期 | [Ch01 · 08 lifetimes](../../02-RFR/Chapter-01-Foundations/08-lifetimes.md) | [10.3 生命周期](../../00-Book/10-generics-traits-lifetimes/) |
| 2.4 闭包 / Iterator | [Ch07 宏 / 闭包语境](../../02-RFR/Chapter-07-Macros/README.md) | [13 闭包与迭代器](../../00-Book/13-iterators-closures/) |
| 2.6 错误 | [Ch04 Error](../../02-RFR/Chapter-04-Error-Handling/README.md) | [09 错误处理](../../00-Book/09-error-handling/) |

---

## 读完本章你能做什么

- 打开 `Vec` / `Option` / `Result` 源码时，能认出 **所有权转移、trait 约束、生命周期标注** 各自在干什么。
- 理解 **`Iterator` + 闭包** 如何支撑 `for`、适配器链与零成本抽象。
- 进入 [第 3 章 容器](../README.md#目录) / [Nomicon 智能指针](../../04-Rust-Nomicon/08_Impl_Vec_Arc/README.md) 前，语法知识已 **成体系**，而非零散 API 记忆。
