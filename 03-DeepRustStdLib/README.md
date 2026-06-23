# 03 · Deep Rust StdLib（标准库进阶）

> Rust 主线：**`00-Book` → `02-RFR` → `01-ER` → 本目录** → [`04-Rust-Nomicon`](../04-Rust-Nomicon/README.md) → [`05-Async-Concurrency-Network`](../05-Async-Concurrency-Network/README.md)

在 **Effective Rust** 之后、**Nomicon** 之前，系统啃 **标准库设计与实现边界** — 为 unsafe / 并发 / 网络专题打底。

---

## 阅读定位

| 阶段 | 仓库 | 侧重 |
|------|------|------|
| 语法底座 | [`00-Book`](../00-Book/Book-本书目录.md) | 所有权、类型、错误、trait |
| 内功 | [`02-RFR`](../02-RFR/RFR-本书目录.md) | 内存模型、类型系统、异步原理 |
| 工程习惯 | [`01-ER`](../01-ER/ER-本书目录.md) | API 设计、惯用写法 |
| **标准库进阶** | **本目录** | `std` 模块语义、容器/IO/并发原语、与源码对照 |
| unsafe 边界 | [`04-Rust-Nomicon`](../04-Rust-Nomicon/README.md) | 布局、型变、未初始化、FFI |
| 实战 | [`05-Async-Concurrency-Network`](../05-Async-Concurrency-Network/README.md) | atomic · tokio · 网络 |

---

## 目录

| 章 | 主题 | 入口 |
|:---:|------|------|
| **1** | Rust 标准库体系概述（`core`/`alloc`/`std` · 模块 · 哲学 · 读源码） | [chapter01_std_overview/](./chapter01_std_overview/README.md) |
| **2** | **Rust 特性小结**（衔接 RFR ↔ `std`：trait / 生命周期 / Iterator / 错误 / 泛型） | [chapter02_rust_features_summary/](./chapter02_rust_features_summary/README.md) |
| **3** | 容器：`Vec` / `String` / `HashMap`（规划） | Book 8 · RFR Ch02 |
| **4** | 智能指针：`Box` / `Rc` / `Arc` / `RefCell`（规划） | Book 15 · Nomicon 08 |
| **5** | 并发原语：`thread` / `sync` / `atomic`（规划） | RFR Ch10 · 05-atomic |
| **6** | I/O：`fs` / `net` / `Read` / `Write`（规划） | 05-rust_network |
| **7** | 错误专题：`Try` / `From` 深挖（规划） | RFR Ch04 · ER Item 04 |

**当前进度**：第 **1** 章 **1.1～1.4** · 第 **2** 章 **2.1～2.7** 已整理。

---

## 相关

- 纯阅读路线（含本目录序号）→ [`docs/纯阅读路线.md`](../docs/纯阅读路线.md)
- 仓库总索引 → [`README.md`](../README.md)
