# 第 7 章：宏 (Macros)

> **Rust for Rustaceans** · [全书目录](../RFR-本书目录.md)  
> 原书章名：**Macros** — 让编译器替你生成代码：`macro_rules!` 声明宏 vs 过程宏 (proc-macros)。

## 本章结构（与原书对齐）

**开篇总览** · **3 个主节** · 连同二级子节共 **10 个部分**（2 个带子的主节标题 + 3 + 4 + Summary）。

| | 笔记 |
|---|------|
| **总览（复习）** | [00 宏核心总览](./00-macros-overview.md) — **§0 直觉：模板 / 模式匹配 / vs Java 注解** |
| **宏分类（4 类）** | [00 宏分类总览](./00-macro-taxonomy.md)（声明 + derive/attr/类函数 · [速记](./00-macro-taxonomy-cheat-sheet.md)） |
| **Token 基础** | [00 Token 与宏展开](./00-token-and-macro-pipeline.md)（词法 · 声明/过程宏 · 编译时序 · [速记](./00-token-cheat-sheet.md)） |
| **宏 vs 函数** | [00 宏 vs 函数](./00-macro-vs-function.md)（八维对比 · 取舍 · [速记](./00-macro-vs-fn-cheat-sheet.md)） |

| 主节 | 英文 | 子节 / 笔记 |
|------|------|-------------|
| **1** | Declarative Macros | [01 何时使用](./01-when-to-use-declarative-macros.md) · [02 如何工作](./02-how-declarative-macros-work.md) · [03 如何编写](./03-how-to-write-declarative-macros.md) |
| **2** | Procedural Macros | [04 过程宏类型](./04-types-of-procedural-macros.md)（derive/属性/类函数 · [04 速记](./04-cheat-sheet.md)）· [05 代价](./05-cost-of-procedural-macros.md) · [06 你真的需要宏吗](./06-so-you-think-you-want-a-macro.md) · [07 如何工作](./07-how-procedural-macros-work.md) |
| **3** | Summary | [08 小结](./08-summary.md) |

## 阅读顺序

```text
00（总览 + Token 链路）→ 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08
```

## 与 The Book / ER 对照

| 主题 | 本仓库 |
|------|--------|
| 宏入门 | [19.5 宏](../../00-Book/19-advanced-features/19.5-宏.md) |
| 可运行 demo | [19.5-macros-demo](../../00-Book/19-advanced-features/19.5-macros-demo/) |
| 展开宏排错 | [第 13 章 · cargo-expand](../Chapter-13-Rust-Ecosystem/01-tools.md) |
| **过程宏动手** | [**proc-macro-workshop 实验**](./proc-macro-workshop-lab.md) → [dtolnay/proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop) |

## 旧版单文件

早期合并稿已拆入上表各文件；见 git 中的 `7-宏-Macros-深度解析.md`。
