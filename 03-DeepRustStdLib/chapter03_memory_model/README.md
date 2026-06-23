# 第 3 章 · 内存模型

> 所属：[03 DeepRustStdLib](../README.md) · 前：[第 2 章 Rust 特性小结](../chapter02_rust_features_summary/README.md) · 后：[第 4 章 容器（规划）](../README.md#目录)

**本章定位**：从 **布局 / 借用 / 内部可变性 / 并发锁 / 幽灵数据 / 泄漏 / 未初始化内存** 十条线读懂 `std` 如何建立在 Rust 内存模型之上 — 为智能指针、Unsafe、HFT 预分配池打底。

**阅读顺序**：**3.1 → 3.2 → … → 3.10**（3.3～3.7 按「单线程内部可变 → 多线程锁」递进）

---

## 子节索引（10 个一级小节）

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **3.1** | 内存布局与对齐 | [3.1-layout-and-alignment.md](./3.1-layout-and-alignment.md) |
| **3.2** | 生命周期与借用检查 | [3.2-lifetimes-and-borrow-check.md](./3.2-lifetimes-and-borrow-check.md) |
| **3.3** | `UnsafeCell` | [3.3-unsafecell.md](./3.3-unsafecell.md) |
| **3.4** | `Cell` | [3.4-cell.md](./3.4-cell.md) |
| **3.5** | `RefCell` | [3.5-refcell.md](./3.5-refcell.md) |
| **3.6** | `Mutex` | [3.6-mutex.md](./3.6-mutex.md) |
| **3.7** | `RwLock` | [3.7-rwlock.md](./3.7-rwlock.md) |
| **3.8** | `PhantomData` | [3.8-phantomdata.md](./3.8-phantomdata.md) |
| **3.9** | 内存泄漏与循环引用 | [3.9-leaks-and-cycles.md](./3.9-leaks-and-cycles.md) |
| **3.10** | `MaybeUninit` | [3.10-maybeuninit.md](./3.10-maybeuninit.md) |

---

## 类型关系速览

```text
编译期借用（& / &mut）
        │
        ▼
3.3 UnsafeCell<T>  ← 所有「规则内可绕过 &T 不变」的根
        │
   ┌────┴────┐
   ▼         ▼
3.4 Cell   3.5 RefCell     （单线程内部可变）
   │
   └─ 3.6 Mutex / 3.7 RwLock  （+ 阻塞 + Send/Sync，跨线程）

3.8 PhantomData  — 不参与布局，标记逻辑所有权/变型
3.9 Rc 环 / leak — 与 3.5/3.6 对照
3.10 MaybeUninit — 未初始化槽位；HFT 内存池关键
```

---

## 与主线对照

| 本章 | 本仓库延伸 |
|------|------------|
| 布局 / 对齐 | [RFR Ch02 layout](../../02-RFR/Chapter-02-Types/02-layout.md) · [Nomicon 02](../../04-Rust-Nomicon/02_Data_Layout/README.md) |
| 借用 / 生命周期 | [RFR Ch01 · 08](../../02-RFR/Chapter-01-Foundations/08-lifetimes.md) · [Ch02 §2.3](../chapter02_rust_features_summary/2.3-lifetimes-in-stdlib.md) |
| UnsafeCell / Cell | [RFR 07-2](../../02-RFR/Chapter-01-Foundations/07-2-unsafecell-and-containers.md) · [07-3 Cell vs RefCell](../../02-RFR/Chapter-01-Foundations/07-3-cell-vs-refcell.md) |
| Mutex / RwLock | [05-atomic Ch1](../../05-Async-Concurrency-Network/01-atomic/Chapter-01-Rust-Concurrency-Basics/) · [RwLock 贯通笔记](../../05-Async-Concurrency-Network/01-atomic/RwLock与读写锁体系-贯通笔记.md) |
| MaybeUninit | [Nomicon 05](../../04-Rust-Nomicon/05_Uninit_Mem/README.md) · [RFR Ch09 §03](../../02-RFR/Chapter-09-Unsafe-Code/03-calling-unsafe-functions.md) |

---

## HFT 阅读提示

| 节 | 实盘关联 |
|----|----------|
| **3.1** | 结构体 padding、缓存行、热路径字段顺序 |
| **3.6/3.7** | 共享行情 / 订单簿的互斥 vs 读写锁选型 |
| **3.9** | `Arc` 环导致泄漏；弱引用 `Weak` 破环 |
| **3.10** | **预分配未初始化内存池**、ring buffer 槽位、避免重复 zero-init |
