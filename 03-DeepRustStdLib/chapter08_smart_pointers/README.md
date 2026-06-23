# 第 8 章 · 智能指针

> 所属：[03 DeepRustStdLib](../README.md) · 前：[第 7 章 内部可变性](../chapter07_interior_mutability/README.md) · 后：[第 9 章 用户态标准库基础](../chapter09_userspace_std_basics/README.md) · 原书目录：[本书目录 § 第 8 章](../本书目录.md#第-8-章--智能指针)

**本章定位**：`Box`、`RawVec`、`Vec`、`Rc`/`Weak`、`Arc`、`Cow`、`LinkedList`、`String` — `alloc` 层堆容器与共享所有权的标准库实现剖析。

**阅读顺序**：**8.1 → 8.2 → … → 8.8**

---

## 子节索引

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **8.1** | `Box<T>` 类型分析 | 📝 规划 |
| **8.2** | `RawVec<T>` 类型分析 | 📝 规划 |
| **8.3** | `Vec<T>` 类型分析 | 📝 规划 |
| **8.3.1** | `Vec<T>` 基础分析 | 📝 规划 |
| **8.3.2** | `Vec<T>` 的 Iterator Trait | 📝 规划 |
| **8.4** | `Rc<T>` 类型分析 | 📝 规划 |
| **8.4.1** | `Rc<T>` 类型的构造函数及析构函数 | 📝 规划 |
| **8.4.2** | `Weak<T>` 类型分析 | 📝 规划 |
| **8.4.3** | `Rc<T>` 的其他函数 | 📝 规划 |
| **8.5** | `Arc<T>` 类型分析 | 📝 规划 |
| **8.5.1** | `Arc<T>` 类型的构造函数及析构函数 | 📝 规划 |
| **8.5.2** | `Weak<T>` 类型分析 | 📝 规划 |
| **8.5.3** | `Arc<T>` 的其他函数 | 📝 规划 |
| **8.6** | `Cow<'a, T>` 类型分析 | 📝 规划 |
| **8.6.1** | `ToOwned` Trait 分析 | 📝 规划 |
| **8.6.2** | `Cow<'a, T>` 分析 | 📝 规划 |
| **8.7** | `LinkedList<T>` 类型分析 | 📝 规划 |
| **8.8** | `String` 类型分析 | 📝 规划 |
| **8.8.1** | 初识 `String` 类型分析 | 📝 规划 |
| **8.8.2** | 格式化字符串分析 | 📝 规划 |

---

## 与主线对照

| 本章 | 本仓库延伸 |
|------|------------|
| `Vec` / `RawVec` | [1.2 alloc 库](../chapter01_std_overview/1.2-alloc-crate.md) · [Nomicon 08](../../04-Rust-Nomicon/08_Impl_Vec_Arc/README.md) |
| `Rc` / `Weak` / 环 | [3.9 泄漏与循环引用](../chapter03_memory_model/3.9-leaks-and-cycles.md) |
| `Arc` + 并发 | [第 11 章 并发](../chapter11_concurrency/README.md) · [05-atomic](../../05-Async-Concurrency-Network/01-atomic/README-学习区.md) |
| `String` | [第 6 章 §6.4](../chapter06_basic_types_continued/README.md) |
| Iterator | [第 5 章 迭代器](../chapter05_iterators/README.md) |

---

## HFT 阅读提示

| 节 | 实盘关联 |
|----|----------|
| **8.2～8.3** | 预分配 `Vec`、控制 `reserve`、避免热路径 realloc |
| **8.5～8.5.3** | 跨线程共享只读行情；`Arc::clone` 原子成本 |
| **8.6** | `Cow` 借视图 vs 按需拥有，减少符号表克隆 |
| **8.8.2** | 格式化日志与 `format!` 分配开销 |
