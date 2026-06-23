# 03 · Deep Rust StdLib（标准库进阶）

> 原书：《深入 Rust 标准库：必备的 Rust 语言高级指南》— 完整 PDF 目录见 **[本书目录.md](./本书目录.md)**  
> Rust 主线：**`00-Book` → `02-RFR` → `01-ER` → 本目录** → [`04-Rust-Nomicon`](../04-Rust-Nomicon/README.md) → [`05-Async-Concurrency-Network`](../05-Async-Concurrency-Network/README.md)

在 **Effective Rust** 之后、**Nomicon** 之前，按原书章节啃 **标准库底层与内存操作** — 为 unsafe / HFT 内存池 / 并发专题打底。

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

## 目录（原书对照）

| 章 | 原书主题 | 入口 | 进度 |
|:---:|----------|------|------|
| **1** | 标准库体系概述（1.1～1.4） | [chapter01_std_overview/](./chapter01_std_overview/README.md) | ✅ 1.1～1.4 |
| **2** | Rust 特征小议（2.1～2.4） | [chapter02_rust_features_summary/](./chapter02_rust_features_summary/README.md) | 📝 按书重构 |
| **3** | **内存操作**（3.1～3.11） | [chapter03_memory_model/](./chapter03_memory_model/README.md) | 📝 按书重构 |
| **4** | 基本类型及基本 Trait（4.1～…） | 📝 `chapter04_primitive_types/` | 规划 |

细目与页码 → **[本书目录.md](./本书目录.md)**（含 HFT 阅读提示、与 RFR/Nomicon/05-atomic 交叉链接）。

**补充轨道**（非原书编号）：ch2 现有 `2.1～2.7` 与附录为 **RFR↔`std` 桥梁**，将逐步收敛或迁入附录；ch3 现有 `UnsafeCell`/`Mutex` 等笔记将迁往 `chapter03_std_sync_supplement/`（规划）。

---

## 相关

- 纯阅读路线 → [`docs/纯阅读路线.md`](../docs/纯阅读路线.md)
- 仓库总索引 → [`README.md`](../README.md)
