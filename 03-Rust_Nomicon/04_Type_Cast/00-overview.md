# 04 · Type Conversions · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[03 Lifetime](../03_Lifetime_Variance/README.md) · 下一章：[05 Uninit Mem](../05_Uninit_Mem/README.md)

---

官方标题 **Type Conversions**。底层编程中，二进制位表示与类型处理至关重要。本章按**从安全自动到极度危险**的顺序讲解转换机制。

| 对照 | 路径 |
|------|------|
| ER 类型转换 | [Item 05](../../01-ER/Chapter-01-Types/Item-05-type-conversions/README.md) |
| Deref / 点运算 | [Book 15.2 Deref](../../00-Book/15-smart-pointers/15.2-通过Deref将智能指针当作引用.md) |
| RFR casting | [08-casting](../../02-RFR/Chapter-09-Unsafe-Code/08-casting.md) |
| 数据布局前提 | [02 Data Layout](../02_Data_Layout/00-overview.md) |

**读完应能回答**：隐式转换何时发生、点运算符的查找顺序、`as` 与 transmute 的危险边界。

---

## 1. 隐式转换 (Coercions)

Rust 在特定上下文中**自动**转换类型，通常是对类型的「弱化」（改变指针类型、缩短生命周期等）。

- 目的：日常代码「开箱即用」，**基本无害**。
- **Trait 匹配时不自动转换**（方法调用的 **receiver** 除外）。

→ 源码：[src/coercions.rs](./src/coercions.rs)（`&String` → `&str`、`&T` → `&dyn Trait`、数组 unsizing）

---

## 2. 点运算符魔法 (The Dot Operator)

`value.foo()` 时，编译器按优先级尝试直到匹配：

1. **按值**调用 `T::foo`
2. **Auto-referencing**：`&T` / `&mut T`
3. **Auto-dereferencing**：`Deref` / `DerefMut` 链
4. **Unsizing**：如 `[T; N]` → `[T]`，以寻找方法

→ 源码：[src/dot_operator.rs](./src/dot_operator.rs)

---

## 3. 显式转换 (Casts)

`expr as Type` — 隐式转换的**超集**。

| 范围 | 说明 |
|------|------|
| 基本数字 | `i32 as u8` 等，可能截断/改变符号 |
| 原生指针 | 运行时通常不报错，但随意 cast 是 UB 温床 |
| **非传递性** | `e as U1 as U2` 合法 **≠** `e as U2` 合法 |

→ 源码：[src/casts.rs](./src/casts.rs)

---

## 4. 内存重释 (Transmutes)

`mem::transmute<T, U>` — **终极黑魔法**：强制把底层比特 reinterpret 为另一类型，**唯一要求**是 `size_of` 相同。

- 作者称此为 Rust 中**最可怕、最不安全**的操作之一，防护形同虚设。
- 误用 → 立即 **UB**。
- **`&T` transmute 成 `&mut T` 永远是 UB**（破坏别名与优化假设）。

→ 源码：[src/transmute.rs](./src/transmute.rs)（仅演示同尺寸无害 reinterpret；危险用法仅注释）
