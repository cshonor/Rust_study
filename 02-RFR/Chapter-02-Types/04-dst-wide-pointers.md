# 1.4 Dynamically Sized Types and Wide Pointers（DST 与宽指针）

> 所属：**Types in Memory** · [← 章索引](./README.md)

前置 → [03 复合类型 · Niche / 枚举](./03-complex-types.md) · 布局 → [02 layout](./02-layout.md)

---

## 一、DST（动态大小类型）是什么

**编译期无法确定 `size_of` 的类型** — 因此：

- **不能**按值直接放栈：`let s: str = "hello";` ❌
- **必须**通过指针 / 引用 / `Box` / `Rc` 等**间接持有**

### Rust 里两类典型 DST

| 类别 | 例子 | 编译期未知的是什么 |
|------|------|-------------------|
| **切片** | `[T]`、`str` | **长度**（元素个数 / 字节数） |
| **Trait 对象底层** | `dyn Trait` | **具体实现类型**的大小 + vtable |

- **`str`**：UTF-8 字节序列的 DST；`&str` = 对 `str` 的宽引用
- **`[T]`**：任意长度连续 `T`；`&[T]`、`Vec<T>` 的视图都涉及长度元数据

**Sized vs DST**：`T: Sized` 才能 `let x: T`、按值传参（默认）；DST 只能出现在 **`&` / `&mut` / `Box` / …** 之后。

---

## 二、宽指针（Fat / Wide Pointer）

指向 **DST** 的引用 / 裸指针 / `Box`，在 **64 位**上常为 **16 字节** = **两个 `usize`**：

```text
瘦指针 (&T, T: Sized)     [  data ptr  ]           8 B

宽指针 (&[T], &str)        [  data ptr  |  len     ]  16 B
宽指针 (&dyn Trait)        [  data ptr  |  vtable  ]  16 B
```

| 宽指针 | 第一字（data pointer） | 第二字（metadata） |
|--------|------------------------|---------------------|
| **`&[T]` / `&str`** | 首元素 / 字符串起始地址 | **长度**（元素数或字节数） |
| **`&dyn Trait`** | 对象实例地址 | **vtable 指针** |

**要点**：`&[T]`、`&dyn Trait` **本身**是 **Sized**（固定 16B）— 不确定大小的是**它们指向的目标类型**。

### 实测（x86_64）

```rust
use std::mem::size_of;

assert_eq!(size_of::<&u32>(), 8);              // 瘦指针
assert_eq!(size_of::<&[u32]>(), 16);           // 宽指针
assert_eq!(size_of::<&str>(), 16);
assert_eq!(size_of::<&dyn std::fmt::Debug>(), 16);
```

→ [`layout-demo`](./layout-demo/) 末尾有 DST 打印

### 64 位字节布局（概念图）

**`&[u32]` 指向 3 个元素：**

```text
offset 0..7:   data ptr  → 堆/栈上第一个 u32
offset 8..15:  len = 3
```

**`&dyn Debug`（trait object）：**

```text
offset 0..7:   data ptr  → 具体 struct 实例
offset 8..15:  vtable ptr → 该类型的虚函数表
```

---

## 三、与 `Vec` / 容器

| | 栈上句柄 | 堆上 payload |
|---|----------|--------------|
| **`Vec<T>`** | `{ ptr, len, cap }` — 24B（64 位） | 连续 `T` |
| **`&[T]`** | 宽指针 `{ ptr, len }` — 16B | 不拥有，只借用 |

`Vec` 的 `{ ptr, len, cap }` 可看作 **拥有型** slice + 容量；设计思路上也是 **数据指针 + 元数据**。

→ [第 1 章 03.1 栈/堆](../Chapter-01-Foundations/03-1-rust-memory-model.md)

---

## 四、HFT / FFI 实战要点

### 1. 性能：宽指针与 `dyn Trait`

| | 影响 |
|---|------|
| **宽指针** | 比瘦指针多带 **len / vtable**；多一次读元数据（通常仍很小） |
| **`dyn Trait` 调用** | **虚表间接跳转** — 难 inline，分支预测更差 |
| **热路径建议** | **静态分发**（泛型 monomorphize、`impl Trait`）替代 `dyn` → [05 编译与分发](./05-compilation-dispatch.md) |

纳秒级路径：少 `dyn`、少额外 indirection；必要时用具体类型或函数指针表（可控 layout）。

### 2. FFI：DST 不能原样过边界

**宽指针 / `dyn Trait` 是 Rust 抽象**，C 侧没有一一对应：

| Rust | 过 FFI 常见写法 |
|------|-----------------|
| `&[T]` / `&str` | `*const T` + `len`（或 `char*` + len 视协议） |
| `&dyn Trait` | ❌ 避免 — 改用 **具体类型** 或 **`extern "C" fn`** |
| `String` | 通常 `*const u8` + len，或 caller-owned buffer |

→ [第 11 章 FFI](../Chapter-11-Foreign-Function-Interfaces/README.md) · layout [02 · repr(C)](./02-layout.md)

### 3. 内存池 / 静态容器

- 自研 arena / ring buffer：常显式存 **`ptr + len`**（同宽指针语义），避免 DST 按值
- 预分配 + 固定 cap → **Sized** 数组或 `(ptr, len, cap)` 三元组，借鉴 `Vec` 句柄

---

## 五、与分发 / 存在类型

| 写法 | 分发 | 运行期开销 |
|------|------|------------|
| **`&dyn Trait` / `Box<dyn Trait>`** | **动态**（vtable） | 有间接调用 |
| **泛型 `T: Trait` / monomorphization** | **静态** | 无 vtable |
| **`impl Trait`（返回位置）** | **静态**（具体类型，隐藏名字） | 无 vtable → [10 存在类型](./10-existential-types.md) |

---

## 易错点

| 易错 | 纠正 |
|------|------|
| `&[T]` 和 `*const T` 一样大 | 切片引用是 **16B 宽指针** |
| `str` 可以 `let s: str` | 只有 **`&str` / `Box<str>`** 等 |
| FFI 传 `&[u8]` | C 收 **指针 + 长度** 两段 |
| HFT 热路径随便 `dyn` | 优先 **静态分发** |

---

## 对照阅读

- [05 编译与分发](./05-compilation-dispatch.md) · [10 存在类型](./10-existential-types.md)
- ER → [Item 12 · dyn Trait 与 DST](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/07-dyn-trait-dst-carriers.md)
- Book → [17.2 trait 对象](../../00-Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md)
- 实测 → [`layout-demo`](./layout-demo/)
