# 1.3 Complex Types（复合类型）

> 所属：**Types in Memory** · [← 章索引](./README.md)

结构体、枚举、元组、数组等在内存中并非「简单拼接比特」——编译器会做**布局与优化**决策。

前置 → [01 对齐](./01-alignment.md) · [02 布局 / repr](./02-layout.md) · 可运行验证 → [layout-demo](./layout-demo/)

---

## 1. 结构体 (`struct`)

Rust 结构体**不是**按源码顺序简单拼接：

| 机制 | 说明 |
|------|------|
| **padding** | 字段间插入填充，满足各自 alignment；struct 总 `size` 是整体 align 的倍数 |
| **字段重排**（默认 `repr(Rust)`） | 编译器**重排**字段，常把 align 大的放前，**减小** `size_of` |
| **`repr(C)`** | **禁用重排**；源码顺序 = 内存顺序 → FFI / 二进制协议 |

### 例子：`DefaultOrder { a: u8, b: u32, c: u16 }`

**x86_64 实测**（[`layout-demo`](./layout-demo/)）：

| | `repr(Rust)` | `repr(C)` |
|---|--------------|-----------|
| `size_of` | **8** | **12** |
| 内存顺序 | **b → c → a**（重排） | **a → b → c**（源码序） |
| offset `a` | 6 | 0 |

```rust
#[derive(Debug)]
struct DefaultOrder { a: u8, b: u32, c: u16 }
// repr(Rust)：b@0, c@4, a@6 → 8 字节

#[repr(C)]
struct DefaultOrderC { a: u8, b: u32, c: u16 }
// repr(C)：a@0, pad 3, b@4, c@8, pad 2 → 12 字节
```

→ 更多 `repr` → [02 布局](./02-layout.md)

---

## 2. 枚举 (`enum`)

### 变体是什么

**变体 (variant)** = 枚举里每一个独立分支 = **标签** + **可选载荷 (payload)**：

```rust
enum Color {
    Red,           // 变体 1：无载荷
    Green(u8),     // 变体 2：1 字节
    Blue(u32, u16) // 变体 3：tuple 载荷（按 tuple layout）
}
```

同一时刻**只有一个变体生效**。

---

### 共享载荷区（联合体）— 大小规则

所有变体的数据放在**同一块共享内存**（类似 C `union`）：

> **共享载荷区大小 = 所有变体载荷大小的最大值**（再按对齐规则向上取整）

```rust
enum Demo {
    A(u8),   // 载荷 1 B
    B(u32),  // 载荷 4 B  ← 最大
    C(u16),  // 载荷 2 B
}
// 共享载荷区 = 4 字节（由 B 的 u32 决定）
// x86_64 实测 size_of::<Demo>() = 8（4 载荷 + 判别式 + 对齐 padding）
```

**无载荷变体**（如 `Red`、`None`）**不增大**载荷区 — 仍由**有数据变体里最大的**决定。

还要加上 **alignment / padding** — 不是简单相加；以 `size_of` 为准 → [`layout-demo`](./layout-demo/)。

---

### 普通枚举整体布局（无 Niche）

```text
+------------------+----------------------+
| 判别式 (tag)      | 共享载荷区 (max 变体)  |
+------------------+----------------------+
```

```rust
enum Either { A(u32), B(u64) }
// x86_64：size_of = 16 — 载荷区要能装 u64 + tag + 对齐
```

---

### `Option<T>` 串起来

```rust
enum Option<T> {
    None,      // 载荷 0
    Some(T),   // 载荷 = size_of::<T>()
}
```

**共享载荷区 = `T` 的大小**（`None` 不占载荷）。

| `T` | 典型 x86_64 | 原因 |
|-----|-------------|------|
| **`u32`** | `Option<u32>` = **8** | `u32` 无「非法值」可编码 `None` → **独立 tag** + 4B 载荷 + 对齐 |
| **`&u32`** | `Option<&u32>` = **8** = `&u32` | **Niche**：`null` = `None`，指针 = `Some` → **无额外 tag** |
| **`NonZeroU32`** | `Option<NonZeroU32>` = **4** = `u32` | **Niche**：`0` = `None` |
| **`Box<u32>`** | `Option<Box<u32>>` = **8** | **Niche**：空指针 = `None` |

```rust
assert_eq!(size_of::<Option<&u32>>(), size_of::<&u32>());
assert_eq!(size_of::<Option<NonZeroU32>>(), size_of::<u32>());
```

---

### Niche optimization（空位优化）

**Niche** = 类型二进制表示里**永远不会出现**的比特模式；编译器用它编码另一变体（如 `None`），**省掉独立判别式**。

| 类型 | Niche（非法模式） | 典型 |
|------|-------------------|------|
| `&T` / `&mut T` | null | `Option<&T>` |
| `NonNull<T>` / `Box<T>` | null | `Option<NonNull<T>>`, `Option<Box<T>>` |
| `NonZeroU32` 等 | `0` | `Option<NonZeroU32>` |
| `fn()` | null | `Option<fn()>` |

**特性**：

1. **零成本抽象** — 编译器自动完成，源码仍写 `Option<T>`
2. **不要依赖具体比特布局** — 实现细节可随版本/平台变；FFI 用 `repr(C)` 等显式固定
3. **`#[repr(transparent)]` 单字段 newtype** — 可**继承**内部类型的 niche

自定义类型要自动 niche 需编译器识别（如 std 的 `NonZero*`）；手写 struct **不会**自动获得 niche。

→ 运行对比 → [`layout-demo`](./layout-demo/) · 宽指针 → [04 DST](./04-dst-wide-pointers.md)

---

### 空枚举 `enum Void {}`

- **无合法值** — uninhabited type
- x86_64 实测：`size_of::<Void>() == 0`（无实例可构造）
- 用于「不可能发生」的分支、`!` 相关类型论

---

## 3. 元组与数组

| 类型 | 内存直觉 |
|------|----------|
| **元组 `(T1, T2, …)`** | 类似**匿名 struct**；按元素顺序布局（默认也可能被优化，以 `size_of` 为准）；有 padding |
| **`[T; N]`** | **连续** N 个 `T`；`size = N × size_of::<T>()`（含末尾对齐）；元素地址连续 |
| **`Vec<T>`** | **栈**上 `{ ptr, len, cap }`；**元素在堆** — 见 [第 1 章 03.1](../Chapter-01-Foundations/03-1-rust-memory-model.md) |

```rust
assert_eq!(size_of::<[u32; 4]>(), 4 * size_of::<u32>());
```

**`Vec` 不是数组**：`Vec` 是智能指针；`[T; N]` 是栈上（或内嵌）固定大小块。

---

## 4. 与 Trait 的衔接

复合类型是 **`impl Trait` 的载体**；行为取决于**字段类型 + 布局**：

| Trait | 规则（摘要） |
|-------|-------------|
| **`Copy`** | 所有字段都是 `Copy` → 可 `#[derive(Copy, Clone)]`；有自定义 **`Drop` 则不能 Copy** |
| **`Clone`** | 可逐字段 clone；`String` 等会深拷贝堆 |
| **`Drop`** | struct：**按源码字段定义顺序** drop（**先定义先 drop**）— **不是**内存重排后的顺序，**也不是**逆序 |

```rust
struct S { a: String, b: String } // 定义顺序 a → b
// drop 时：先 drop a，再 drop b（与局部变量 LIFO 逆声明不同！）
```

→ Drop 细节 [第 1 章 · 04 所有权](../Chapter-01-Foundations/04-ownership.md)

---

## 5. 控制布局的 `repr`（小结）

| `repr` | 用途 |
|--------|------|
| **`Rust`（默认）** | 日常代码；可重排、省空间 |
| **`C`** | FFI、固定二进制 |
| **`transparent`** | 单字段 newtype，与内部字段同 layout |
| **`packed`** | 极致紧凑；**misalign / UB 风险** |

验证工具（**stable**）：

```rust
use std::mem::{align_of, offset_of, size_of};
```

---

## 易错点

| 易错 | 纠正 |
|------|------|
| struct 字段 Drop = 逆声明 | **正定义序** drop；逆声明是**多个局部变量** |
| struct Drop 顺序 = 内存重排顺序 | Drop 按**源码字段顺序**，不是 `offset_of` 顺序 |
| `Option<&T>` 比 `&T` 大 | niche 下常**一样大** |
| 元组/struct 默认 = 源码顺序 | 只有 **`repr(C)`** 保证 |
| `offset_of` 要 nightly | **stable** 可用 `offset_of!` |

---

## 对照阅读

- Book → [5.1 结构体](../../00-Book/05-structs/5.1-定义结构体.md) · [6.1 枚举](../../00-Book/06-enums-pattern-matching/6.1-定义枚举.md)
- 运行验证 → [`layout-demo`](./layout-demo/)
- 下一节 → [04 DST 与宽指针](./04-dst-wide-pointers.md)
