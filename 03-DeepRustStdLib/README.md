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

## 目录规划（待补）

笔记按 `std` 子系统分章，与 The Book / RFR 交叉索引：

| 章（规划） | 主题 | 对照 |
|------------|------|------|
| **01** | 容器：`Vec` / `String` / `HashMap` 行为与复杂度 | Book 8 · RFR Ch02 |
| **02** | 智能指针：`Box` / `Rc` / `Arc` / `RefCell` | Book 15 · Nomicon 08 |
| **03** | 并发原语：`thread` / `sync` / `atomic` 入口 | RFR Ch10 · 05-atomic |
| **04** | I/O：`fs` / `net` / `io::Read` / `Write` | 05-rust_network |
| **06** | 错误与类型转换：`Error` / `From` / `Try` | RFR Ch04 · ER Item 04 |

---

## 相关

- 纯阅读路线（含本目录序号）→ [`docs/纯阅读路线.md`](../docs/纯阅读路线.md)
- 仓库总索引 → [`README.md`](../README.md)
