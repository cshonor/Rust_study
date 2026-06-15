# 05 · Working With Uninitialized Memory · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[04 Type Cast](../04_Type_Cast/README.md) · 下一章：[06 OBRM](../06_OBRM_RAII/README.md)

---

官方标题 **Working With Uninitialized Memory**。运行时分配的内存初始均为**未初始化**；在未初始化时将其解释为任何类型的值 → **UB**。本章讲解 Safe / Unsafe 两侧如何管理这些区域。

| 对照 | 路径 |
|------|------|
| RFR unsafe 调用 | [03-calling-unsafe-functions](../../02-RFR/Chapter-09-Unsafe-Code/03-calling-unsafe-functions.md) |
| no_std 动态内存 | [02-dynamic-memory-allocation](../../02-RFR/Chapter-12-Rust-Without-Standard-Library/02-dynamic-memory-allocation.md) |
| 实现 Vec（后续） | [09_Impl_Vec_Arc](../09_Impl_Vec_Arc/README.md) |

**读完应能回答**：Safe 如何静态阻止读未初始化栈变量、Drop flags 解决什么、`MaybeUninit` 与 `ptr::write` 的分工。

---

## 1. 安全受检的未初始化内存 (Checked)

### 静态分析

栈变量在显式赋值前未初始化；Rust 用**基本分支分析**在编译期阻止赋值前读取。

→ 源码：[src/checked.rs](./src/checked.rs)（条件初始化、`if/else` 全路径赋值）

### 逻辑未初始化

非 `Copy` 值被**移出**后，原变量在逻辑上重新变为未初始化，不可再使用。

→ 源码：[src/checked.rs](./src/checked.rs)（move 后不可再用 — 注释 + 编译器 enforced）

---

## 2. 丢弃标志 (Drop Flags)

条件初始化 / 去初始化 / 重新赋值时，离开作用域或覆盖变量是否应 **Drop** 旧值？Rust 在栈上用 **drop flags** 跟踪初始化状态。

| 情况 | 行为 |
|------|------|
| 路径复杂、编译期无法确定 | **运行时**检查 flag 再决定是否 Drop |
| 可静态推导 | 优化为**无运行时开销** |

→ 源码：[src/drop_flags.rs](./src/drop_flags.rs)（条件构造带 `Drop` 的类型）

---

## 3. 非受检的未初始化内存 (Unchecked)

Safe Rust **不允许**数组部分初始化 — 对逐元素构建的底层结构过于死板，须借助 Unsafe。

### `MaybeUninit<T>`

标准工具：向编译器标明「可能未初始化」。

- Drop `MaybeUninit` **不会**清理内部（因可能无合法值）
- 覆盖时**不会**误 Drop 旧值
- 确认全部合法后，`assume_init` / `assume_init_read` 或 transmute 转为普通类型

→ 源码：[src/maybe_uninit.rs](./src/maybe_uninit.rs)

### `ptr` 模块

| API | 作用 |
|-----|------|
| `ptr::write` | 盲写目标地址，**绕过**旧值 Drop |
| `ptr::copy` | 类似 C `memmove`（可重叠） |
| `ptr::copy_nonoverlapping` | 类似 C `memcpy` |

滥用极易 UB → 源码：[src/ptr_ops.rs](./src/ptr_ops.rs)

### 废弃的 `mem::uninitialized`

旧代码可能见到；与语言机制**无法修复的冲突**，新代码**永远**用 `MaybeUninit` 替代（见 `00-overview` 与源码注释）。

---

## 4. 与后续章节的关系

`Vec` 的容量增长、部分提交元素，是本章 API 的典型组合：`MaybeUninit` 数组 + `ptr::write` + `set_len` → 见 ch09。
