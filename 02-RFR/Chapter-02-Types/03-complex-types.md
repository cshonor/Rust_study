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

> **一句话**：枚举在内存里通常是 **判别式（标签）+ 联合体（共享载荷）**；Niche 优化时 **标签融进载荷**，省掉独立 tag。

---

### 大白话：三个词

| 词 | 日常说法 | 干什么 |
|----|----------|--------|
| **判别式 (discriminant)** | **标签、记号** | 标记「当前是哪个变体」— `Some` 还是 `None` |
| **联合体 (union)** | **一块共享储物区** | 各变体的数据**不分开存**，共用同一片内存 |
| **载荷 (payload)** | **变体里的实际数据** | 如 `Some(123)` 里的 `123` |

**为什么能共用？** 枚举**同一时刻只会是一个变体**，不可能同时又是 `Some` 又是 `None` — 和 C 里 `union`「同一时刻只活一个成员」一样。

---

### 标准模型（无 Niche）：`Option<T>`

```rust
enum Option<T> {
    Some(T),
    None,
}
```

内存（概念上）= **独立标签** + **联合体（存载荷）**：

```text
+-------------+------------------+
| 判别标签     | 联合体（载荷 T）   |
+-------------+------------------+
  0=None, 1=Some    Some 时存 T
                    None 时闲置
```

**`Option<u32>`（x86_64 实测）**：

- 载荷 `u32`：**4 字节**
- 标签 + 对齐 padding：再占一部分  
- **合计 `size_of = 8`**（不是 4+4 简单心算就完，但量级如此）

标签告诉你「有还是没有」；联合体里放真正的 `u32`。

**泛型实参 `T` 与单态化**：`Option<T>` 不是「一个带参数的内存模板」— 每个具体 `T`（`u32`、`u64`、…）编译后都是**独立枚举类型**，各自算 tag + 联合体 layout。`Option<u32>`（8B）与 `Option<u64>`（通常 16B）**不共用**载荷区尺寸。跨 crate 文件、相同 `T` 则**复用**同一份 layout。→ [05 单态化与内存](./05-compilation-dispatch.md#二单态化与内存不同-t--不同类型)

---

### 为什么是「联合体」？

```rust
enum Demo { A(u8), B(u32), C(u16) }
// 载荷区大小 = max(1, 4, 2) = 4 字节 — 见下文
```

三个变体**不会各占一块**，而是**抢同一块 4 字节储物区** — 谁被选中谁用。  
x86_64 整个 `Demo` 还要再加 **tag** → 实测 **8 字节**（[`layout-demo`](./layout-demo/)）。

---

### Niche 优化：标签「消失」

有些类型的比特模式里，有**永远不会出现的非法值（Niche）** — 如 `&u32` **绝不**是 null。

编译器约定：

- **null 指针** → `None`
- **非 null** → `Some(引用)`

**独立标签格删掉**，指针**兼任**标签 + 载荷：

```text
优化前 Option<&u32>:  [ tag ] + [ 指针 8B ]  → 可能更大
优化后 Option<&u32>:  [ 指针 8B，null=None ] → 与 &u32 同大
```

x86_64：`size_of::<Option<&u32>>() == size_of::<&u32>() == 8`。

---

### 收纳盒类比（串起来）

| | 普通（如 `Option<u32>`） | Niche（如 `Option<&T>`） |
|---|--------------------------|---------------------------|
| 盒子 | **两格**：小卡片（标签）+ 储物区（载荷） | **一格**：储物区自带规则 |
| 规则 | 卡片写「有/无」 | 「正常指针绝不空」→ 空 = 无，非空 = 有 |
| 结果 | 盒子更大（8B） | 盒子不膨胀（= 指针大小） |

---

### 变体是什么（正式一点）

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

### 一句话总结

1. **原生枚举**：**独立判别标签 + 联合体载荷** — 标签选变体，联合体存数据。  
2. **Niche 优化**：用载荷类型自带的**非法比特**顶替标签 → **只剩联合体**，零额外开销（逻辑上）。  
3. **别依赖具体比特** — 实现随编译器/平台变；FFI 用 `repr(C)` 等显式固定。

---

### Niche 是什么？（先字面 → 再原理 → 看效果）

#### 1. 字面：niche = 夹缝、空位

英文 **niche** = **夹缝、空位、没人占的位置**。

在内存里：

> 某类型的二进制表示中，存在**永远不会合法出现**的取值 = **内存空位（Niche）**

编译器**复用这个「废值」**来编码枚举的另一态（如 `None`），省掉独立判别式 → **Niche 优化**。

---

#### 2. 原理：钥匙盒比喻（一秒懂）

盒子只放钥匙；**正常钥匙永远不会是「空盒子」**。

要表达两种状态：**有钥匙（Some）** / **没钥匙（None）**。

| 做法 | 怎么做 | 开销 |
|------|--------|------|
| **普通** | 盒里放钥匙 + **额外贴标签**「有/无」 | 多占标签空间 |
| **Niche** | **空盒 = None**，**非空 = Some** — 不贴标签 | **零额外空间** |

Rust 里：`&T` 合法时**绝不**是地址 `0` → **`0` 就是 Niche**。

---

#### 3. 效果：`Option<T>` 两场景

```rust
enum Option<T> { Some(T), None }
```

**场景 A：`T = u32`（无 Niche）**

- `0..=u32::MAX` **全是合法值**，没有「废值」可用  
- 编译器：**载荷 4B + 独立判别式** → x86_64 **`Option<u32>` = 8**

**场景 B：`T = &u32`（有 Niche）**

- 合法引用 **≠ null** → **地址 0 = None**，非 0 = Some  
- **无独立 tag** → **`Option<&u32>` = 8 = `&u32`**

**场景 C：`T = NonZeroU32`（0 是 Niche）**

- 合法值 **≠ 0** → **0 = None**  
- **`Option<NonZeroU32>` = 4 = `u32` 大小**

---

#### 4. 一句话

> **Niche 优化 = 用类型「永远不会出现的非法取值」兼任枚举状态标记，省去专门判别式，实现零额外内存（相对载荷类型）。**

---

#### 5. 谁自带 Niche？（速查）

| 类型 | 非法值（Niche） | `Option<…>` 是否膨胀 |
|------|-----------------|----------------------|
| `&T` / `&mut T` | null (0) | ❌ 通常不 |
| `Box<T>` / `NonNull<T>` | null | ❌ 通常不 |
| `NonZeroU32` 等 | 0 | ❌ 通常不 |
| `fn()` | null | ❌ 通常不 |
| **`u32` 等普通整数** | 无 | ✅ **`Option<u32>` 更大** |

---

#### 6. 跑一遍（layout-demo 已含）

```rust
use std::mem::size_of;
use std::num::NonZeroU32;

println!("u32: {}", size_of::<u32>());                    // 4
println!("Option<u32>: {}", size_of::<Option<u32>>());      // 8
println!("Option<&u32>: {}", size_of::<Option<&u32>>());    // 8 (= &u32)
println!("Option<NonZeroU32>: {}", size_of::<Option<NonZeroU32>>()); // 4
```

```bash
cargo run --manifest-path 02-RFR/Chapter-02-Types/layout-demo/Cargo.toml
```

**注意**：Niche 是**编译器内部优化** — 不要依赖具体比特布局做协议；FFI 用 `repr(C)` 等显式约定。

**其它**：`#[repr(transparent)]` 单字段 newtype 可**继承**内部类型的 niche；手写 struct **不会**自动 niche。

→ 宽指针 → [04 DST](./04-dst-wide-pointers.md)

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
| `Option<&T>` 比 `&T` 大 | niche 下常**一样大**；`Option<u32>` 常**比 u32 大**（无 niche） |
| 枚举大小 = 所有变体之和 | **max 载荷** + tag + 对齐；不是相加 |
| 无数据变体也占载荷区 | **不占**；载荷区仍由**最大有数据变体**决定 |
| 元组/struct 默认 = 源码顺序 | 只有 **`repr(C)`** 保证 |
| `offset_of` 要 nightly | **stable** 可用 `offset_of!` |

---

## 对照阅读

- Book → [5.1 结构体](../../00-Book/05-structs/5.1-定义结构体.md) · [6.1 枚举](../../00-Book/06-enums-pattern-matching/6.1-定义枚举.md)
- 运行验证 → [`layout-demo`](./layout-demo/)
- 下一节 → [04 DST 与宽指针](./04-dst-wide-pointers.md)
