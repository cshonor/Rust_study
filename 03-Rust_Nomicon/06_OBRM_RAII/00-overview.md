# 06 · The Perils Of OBRM · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[05 Uninit Mem](../05_Uninit_Mem/README.md) · 下一章：[08 Concurrency](../08_Concurrency_Atomic/README.md)

---

官方标题 **The Perils Of Ownership Based Resource Management (OBRM / RAII)**。获取资源常伴随对象创建，释放依赖销毁。本章探讨该机制在底层编程中的挑战与风险。

| 对照 | 路径 |
|------|------|
| Drop / RAII | [Book 15.3 Drop](../../00-Book/15-smart-pointers/15.3-使用Drop运行清理代码.md) |
| ManuallyDrop | [15.3.1](../../00-Book/15-smart-pointers/15.3.1-Drop顺序与进阶场景.md) |
| ER Item 11 | [drop-raii](../../01-ER/Chapter-02-Traits/Item-11-drop-raii/README.md) |
| catch_unwind | [10_FFI](../10_FFI/00-overview.md) §7 |

**读完应能回答**：Rust 构造/析构与 C++ 有何不同、为何 `forget` 仍算内存安全、代理类型泄漏为何危险、Guard 如何保异常安全。

---

## 1. 构造与析构 (Constructors and Destructors)

### 极简构造

无 C++ 式拷贝/默认/移动构造分族；类型不「关心」自己在哪块内存——移动只是 **memcpy**，故无法安全实现纯栈上**侵入式链表**等须感知地址的结构。

### 递归析构

`Drop` 自动析构；`drop` 方法体执行后**递归**清理各字段。可用 **`Option::take`** 或 **`ManuallyDrop`** 打破/延迟默认递归清理。

→ 源码：[src/construct_drop.rs](./src/construct_drop.rs)

---

## 2. 内存泄漏与 `mem::forget` (Leaking)

Safe Rust **并非**绝对无泄漏——死锁、`Rc` 循环引用等均可导致资源永不释放。

`mem::forget` 消耗值但**不调用 Drop**；官方认定这不违背**内存安全**（只是泄漏资源）。

→ 源码：[src/forget_leak.rs](./src/forget_leak.rs)

---

## 3. 代理类型的致命危险 (Proxy Types)

泄漏普通 `Box` 顶多浪费内存；泄漏**代理对象**可能破坏内存安全：

| 类型 | 风险 |
|------|------|
| **`vec::Drain`** | 迭代中途 `forget` → 元素状态不一致；std 用 **leak amplification** 妥协保容器基本安全 |
| **`Rc`** | 恶意 `forget` 克隆体 → 引用计数溢出 → 提前释放 → **UAF** |
| **`thread::scoped`（已移除）** | 守卫被 `forget` → 父栈销毁而子线程仍运行 → **UAF**；API 已废弃重设计 |

→ 源码：[src/proxy_types.rs](./src/proxy_types.rs)（Drain + 注释说明 Rc/scoped）

---

## 4. 栈展开与异常安全 (Unwinding)

`panic!` 默认**栈展开**，沿途 Drop 所有对象。Unsafe 代码常调用用户提供的 `Ord`/`Clone` 等——**随时可能 panic**。

须保证至少 **minimal exception safety**：无论如何 panic，**不破坏内存安全**。

对策：RAII **Guard / Hole**——处理中途 panic 时由 Drop 恢复状态，避免 **double-drop**。

→ 源码：[src/panic_guard.rs](./src/panic_guard.rs)

---

## 5. 数据投毒 (Poisoning)

无法在 panic 后恢复完美逻辑一致性时，用**投毒**阻断坏状态蔓延。例：`Mutex` 持锁期间 panic → 锁被 **poison** → 后续 `lock()` 默认失败。

→ 源码：[src/poisoning.rs](./src/poisoning.rs)
