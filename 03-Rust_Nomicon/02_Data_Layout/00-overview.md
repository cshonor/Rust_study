# 02 · Data Representation in Rust · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[01 Safe Unsafe](../01_Safe_Unsafe/README.md) · 下一章：[03 Lifetime](../03_Lifetime_Variance/README.md)

---

官方标题 **Data Representation in Rust**。底层编程中，数据在内存中的布局至关重要，并深度影响语言其它特性。本章介绍 Rust 如何处理对齐、尺寸，以及如何用 `repr` 控制布局。

| 对照 | 路径 |
|------|------|
| RFR 布局详解 | [02-layout.md](../../02-RFR/Chapter-02-Types/02-layout.md) |
| RFR 实测 demo | [layout-demo](../../02-RFR/Chapter-02-Types/layout-demo/) |
| OS 内存分区 | [03-2-os-memory-layout](../../02-RFR/Chapter-01-Foundations/03-2-os-memory-layout.md) |

**读完应能回答**：`repr(Rust)` 与 `repr(C)` 有何不同、DST/ZST/空类型是什么、何时用哪种 `repr`。

---

## 1. 默认数据布局：`repr(Rust)`

### 对齐与填充 (Alignment and Padding)

所有类型都有按字节指定的**对齐要求**。为保证各字段正确对齐，Rust 会在字段之间自动插入 **padding** 字节。

### 字段重排 (Field Reordering)

与 C 不同，Rust **默认不保证** struct 字段在内存中的书写顺序（数组除外，始终密集且按序排列）。编译器会**重排字段**以消除对齐浪费，因此不同泛型实例可能有截然不同的布局。

→ 源码对照：[src/repr_rust.rs](./src/repr_rust.rs)（`repr(Rust)` vs `repr(C)` 的 `size_of` / `offset_of`）

### 空指针优化 (Null Pointer Optimization)

对某些枚举，若含单元变体（如 `None`）与**不可为空指针**变体（如 `Some(&T)`），Rust 可省去区分变体的 tag：直接将空指针解释为 `None`，使 `Option<&T>` 与 `&T` **同尺寸**。

→ 源码对照：[src/repr_rust.rs](./src/repr_rust.rs)（`Option<&T>` niche）

---

## 2. 尺寸特殊的类型 (Exotically Sized Types)

### 动态大小类型 (DSTs)

大小与对齐在**编译期未知**的类型：特征对象（`dyn Trait`）、切片（`[T]`、`str`）。不能直接按值存储，只能位于指针之后；指针须携带额外信息（切片长度、vtable 等），称为**胖指针 (wide pointer)**。

→ 源码对照：[src/exotic.rs](./src/exotic.rs)

### 零大小类型 (ZSTs)

完全不占内存的类型（如 `struct Nothing;`）。load/store 在底层为 **no-op**。Safe Rust 可忽略；**Unsafe** 中须注意：对 ZST 的指针偏移也是 no-op，且分配器通常要求**非零**分配大小。

### 空类型 (Empty Types)

无法实例化的类型，如 `enum Void {}`。用于类型系统表达不可达状态，例如 `Result<T, Void>` 在类型层面保证**绝不会**返回 `Err`。

→ 源码对照：[src/exotic.rs](./src/exotic.rs)

---

## 3. 替代的数据表示 (Alternative representations)

| `repr` | 作用 | 注意 |
|--------|------|------|
| **`repr(C)`** | 按 C/C++ 规则排序、算 size/align | FFI、内存重释、固定二进制格式 **必用** |
| **`repr(transparent)`** | 仅含一个非 ZST 字段时，布局/ABI 与内层字段完全一致 | 新type 包装 |
| **`repr(u*)` / `repr(i*)`** | 无字段枚举的底层整数大小与符号 | C 枚举互操作 |
| **`repr(packed)`** | 剥离 padding，整体对齐到 1 字节 | **危险**：未对齐访问，ARM 等可能硬件异常 |
| **`repr(align(n))`** | 最小对齐至少为 `n`（2 的幂） | 缓存行对齐、避免 false sharing |

→ 源码对照：[src/repr_alt.rs](./src/repr_alt.rs)
