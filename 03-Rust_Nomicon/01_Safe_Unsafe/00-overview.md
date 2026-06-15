# 01 · Meet Safe and Unsafe · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 下一章：[02 Data Layout](../02_Data_Layout/README.md)

---

官方标题 **Meet Safe and Unsafe**。正文无「第一章」编号，但这是全书第一个核心主题板块，为后续 unsafe 编程建立心智模型。

| 对照 | 路径 |
|------|------|
| The Book unsafe 入门 | [19.1 不安全 Rust](../../00-Book/19-advanced-features/19.1-不安全Rust.md) |
| RFR 深入 | [第 9 章 Unsafe Code](../../02-RFR/Chapter-09-Unsafe-Code/README.md) |
| ER 能不用就不用 | [Item 16 avoid unsafe](../../01-ER/Chapter-03-Concepts/Item-16-avoid-unsafe/README.md) |

**读完应能回答**：Safe 与 Unsafe 的边界在哪、`unsafe` 关键字两种用法、Unsafe 额外五种能力、信任为何不对称、为何安全性是非局部的。

---

## 1. Safe Rust 与 Unsafe Rust 的双重世界

Rust 可视为两种语言的结合：

| | Safe Rust | Unsafe Rust |
|---|-----------|-------------|
| 定位 | 真正的 Rust 语言 | 与 Safe 语义相同，但开放额外能力 |
| 保证 | 类型安全、内存安全；无悬垂指针、use-after-free、UB | 无上述保证——误用即 UB |
| 用途 | 日常开发 | 系统级底层控制（类似 C） |

## 2. `unsafe` 关键字的两种用法

`unsafe` 是安全与非安全世界的**边界**：

1. **声明契约（caller/implementor 责任）**  
   在函数或 trait **声明**上加 `unsafe`，表示调用者或实现者必须阅读文档并手动保证前提条件成立。  
   例：`unsafe fn`、`unsafe trait Send`。

2. **履行契约（程序员已验证）**  
   在代码块或 trait **实现**上加 `unsafe`，表示程序员已确认此处所有 unsafe 操作合法、符合契约。  
   例：`unsafe { ... }`、`unsafe impl Send for MyType`。

→ 源码对照：[src/five_powers.rs](./src/five_powers.rs)

## 3. Unsafe Rust 的 5 种额外能力

仅此五项；**任意一项误用 → 未定义行为 (UB)**：

1. 解引用**原生指针**（raw pointers）
2. 调用 **`unsafe` 函数**（含 FFI、编译器 intrinsics）
3. 实现 **`unsafe` trait**（如 `Send`、`Sync`）
4. 访问或修改**可变静态变量**（`static mut`）
5. 访问 **`union` 字段**

## 4. 信任不对称原则

- **Safe → Unsafe**：Safe Rust 必须**无条件信任** Unsafe 代码被正确编写。
- **Unsafe → Safe**：Unsafe 代码**不能**假设用户提供的 Safe 代码一定正确。

例：`BTreeMap` 内部用 unsafe，若用户自定义 `Ord` 有逻辑 bug，unsafe 层必须足够健壮——**不能**因此产生内存 UB，最多 panic 或逻辑错误。

## 5. 安全性的非局部性（Non-locality）

**本章最重要洞察**：修改一段看似无害的 Safe 代码，也可能让整个库 **unsound** 并触发 UB。

原因：unsafe 正确性依赖程序其他处的 invariant（如 `Vec` 的 `len`/`cap`）。  
**对策**：用**模块可见性（privacy）**在边界封装，对外只暴露 Safe API。

→ 源码对照：[src/privacy.rs](./src/privacy.rs)
