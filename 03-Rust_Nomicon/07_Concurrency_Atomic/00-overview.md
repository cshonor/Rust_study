# 07 · Concurrency and Parallelism · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[06 OBRM](../06_OBRM_RAII/README.md) · 下一章：[08 Impl Vec/Arc](../08_Impl_Vec_Arc/README.md)

---

官方标题 **Concurrency and Parallelism**。Rust 不预设「消息传递 vs 绿色线程」等底层立场，但类型系统使**线程安全抽象**相对易写。本章三主题：数据竞争、Send/Sync、原子与内存序。

| 对照 | 路径 |
|------|------|
| Send / Sync | [Book 16.4 Send与Sync](../../00-Book/16-fearless-concurrency/16.4-Send与Sync.md) |
| RFR 并发 | [Ch10 Concurrency](../../02-RFR/Chapter-10-Concurrency-and-Parallelism/README.md) |
| 实现 Arc（后续） | [08_Impl_Vec_Arc](../08_Impl_Vec_Arc/README.md) |

**读完应能回答**：数据竞争 vs 竞争条件、Send/Sync 含义、Relaxed/Acquire-Release/SeqCst 如何选用。

---

## 1. 数据竞争与竞争条件

### 消除数据竞争

Safe Rust **绝对保证**无**数据竞争**（多线程并发访问同一内存，至少一方写，且无同步）→ **UB**。

根本机制：所有权 + 借用（`&mut` 不可别名）。

### 不保证消除一般竞争条件

**死锁**、逻辑资源竞争等——在通用 OS 环境下**数学上无法彻底防范**；这类问题只导致逻辑错误，不算内存 UB。

### 安全边界

竞争条件 + **`unsafe`** 可能破坏内存安全。经典 **TOCTOU**：

1. 线程 A：边界检查通过  
2. 线程 B：修改长度/索引  
3. 线程 A：`get_unchecked` → **越界 UB**

→ 源码：[src/data_races.rs](./src/data_races.rs)（安全 `get` vs TOCTOU 注释）

---

## 2. 线程安全标记：`Send` 和 `Sync`

**Unsafe marker traits**，并发故事基石：

| Trait | 含义 |
|-------|------|
| **`Send`** | 值**移动**到另一线程安全 |
| **`Sync`** | 多线程**共享**安全；等价于 **`&T: Send` 则 `T: Sync`** |

复合类型字段均满足则自动实现。重要**非线程安全**例外：

- **原生指针** — 无护栏  
- **`UnsafeCell`** — 无同步的内部可变性  
- **`Rc`** — 非原子 refcount，多线程 clone 可溢出 / UAF  

→ 源码：[src/send_sync.rs](./src/send_sync.rs)

---

## 3. 原子操作与内存模型

继承 **C++20** 原子内存模型，弥合开发者期望、编译器优化与硬件行为。

### 重排

编译器优化 + 多核缓存 → 线程间**不保证**普通访问的全局顺序。

### 普通访问 vs 原子访问

仅靠普通 load/store **无法**正确同步；须 **原子访问** 限制重排。

### Memory Orderings

| 排序 | 作用 | 典型场景 |
|------|------|----------|
| **`SeqCst`** | 全局一致顺序，最强、最慢 | 需要全局全序时 |
| **`Release` / `Acquire`** | 释放/获取配对，建立跨线程 happens-before | 锁、发布-订阅 |
| **`Relaxed`** | 仅该操作原子，无跨线程顺序 | 孤立计数器 |

→ 源码：[src/atomics.rs](./src/atomics.rs)
