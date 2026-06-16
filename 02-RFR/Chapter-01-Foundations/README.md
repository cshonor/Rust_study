# 第 1 章：基础 (Foundations)

> **Rust for Rustaceans** · [全书目录](../RFR-本书目录.md)  
> 原书章名：**Foundations** — 从内存模型与类型系统重建所有权、借用、生命周期的心理模型。

## 本章结构（与原书对齐）

**4 个主节** · 连同二级子节共 **11 个部分**（2 个带子的主节标题 + 3 + 1 + 4 + 1）。

| 主节 | 英文 | 子节 / 笔记 |
|------|------|-------------|
| **1** | Talking About Memory | [01 内存术语](./01-memory-terminology.md) · [02 变量深入](./02-variables-in-depth.md) · [03 内存区域](./03-memory-regions.md)（[03.1 Rust 模型](./03-1-rust-memory-model.md) · [03.2 OS/LLVM 布局](./03-2-os-memory-layout.md)） |
| **2** | Ownership | [04 所有权](./04-ownership.md)（[04.1](./04-1-three-rules.md) · [04.2](./04-2-move-copy-clone.md) · [04.3](./04-3-drop.md) · [04.4](./04-4-drop-order.md) · [04.5](./04-5-refs-and-panic.md) · [04.6](./04-6-pitfalls.md)） |
| **3** | Borrowing and Lifetimes | [05](./05-shared-references.md) · [06](./06-mutable-references.md) · [07](./07-interior-mutability.md) · [08](./08-lifetimes.md) · [05–08 速记](./05-08-borrowing-lifetimes-cheat-sheet.md) |
| **4** | Summary | [09 小结](./09-summary.md) |

## 阅读顺序

```text
01 → 02 → 03 → 03.1 → 03.2 → 04 → 05 → 06 → 07 → 08 → 09
```

（03 为索引；03.1 Safe Rust 三分类，03.2 OS/LLVM 五分区 — 可按需跳过 03.2。）

## 与 The Book / ER 对照

| 主题 | 本仓库 |
|------|--------|
| 所有权、移动 | [4.1 什么是所有权](../../00-Book/04-ownership/4.1-什么是所有权.md) |
| 引用与借用 | [4.2 引用与借用](../../00-Book/04-ownership/4.2-引用与借用.md) |
| 生命周期 | [10.3 生命周期](../../00-Book/10-generics-traits-lifetimes/10.3-生命周期与引用有效性.md) |
| RefCell / 内部可变 | [15.5 RefCell](../../00-Book/15-smart-pointers/15.5-RefCell与内部可变性.md) |
| 类型表达结构 | [ER Item 01](../../01-ER/Chapter-01-Types/Item-01-express-data-structures/README.md) |
| 生命周期（ER） | [ER Item 14](../../01-ER/Chapter-03-Concepts/Item-14-lifetimes/README.md) |

## 旧版单文件

早期合并稿已拆入上表各文件；如需对照历史结构见 git 中的 `1-基础-Foundations-深度解析.md`。
