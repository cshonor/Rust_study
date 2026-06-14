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

## 三、`dyn Trait` 底层：胖指针 + vtable

### 3.1 `dyn Trait` 是什么

**动态分发的核心** + **DST** — 编译期不知具体类型大小，**不能**按值使用：

```rust
// ❌ let obj: dyn Animal = Cat;
// ✅ 必须间接持有
let obj: &dyn Animal = &cat;
let boxed: Box<dyn Animal> = Box::new(cat);
```

`&dyn Trait` / `Box<dyn Trait>` 的**句柄**是 Sized（64 位下引用 **16B**）；**指向的** `dyn Trait` 仍是 DST。

### 3.2 胖指针布局（64 位）

```text
&dyn Animal
┌─────────────────────┬─────────────────────┐
│ data ptr (8B)       │ vtable ptr (8B)     │
└─────────────────────┴─────────────────────┘
         ↓                       ↓
   Cat / Dog 实例          Cat 的 Animal vtable（静态只读）
```

与 slice 胖指针**同形状**（两字），第二字含义不同：**len vs vtable**。

### 3.3 vtable 里有什么（概念模型）

编译器为每个 **`(具体类型, Trait)`**（+ 必要 supertrait）生成**静态只读** vtable。布局是 **rustc 内部细节**，下面为**学习用简化**，字段顺序/额外项以编译器为准：

```text
vtable for (Cat, Animal):
┌──────────────────────────────────────────┐
│ drop_in_place(ptr)     // 析构此具体类型   │
│ size_of::<Cat>()       // 对象字节数       │
│ align_of::<Cat>()      // 对齐要求         │
│ Animal::make_sound     // trait 方法 fn 指针│
│ Animal::get_name       // …               │
│ (supertrait 方法，如 Debug::fmt)          │
└──────────────────────────────────────────┘
```

| 部分 | 作用 |
|------|------|
| **drop / size / align** | `Box<dyn Trait>` drop、realloc、`size_of_val` 等需要知**具体类型**布局 |
| **trait 方法指针** | 动态调用时查表跳转 |
| **supertrait** | `trait Animal: Debug` → vtable 也含 `Debug` 方法槽位 |

```rust
// 教学用伪结构 — 非官方 ABI
struct VTableConcept {
    drop_in_place: fn(*mut ()),
    size: usize,
    align: usize,
    // 后跟各 trait 方法的 fn 指针…
}
```

### 3.4 方法调用：`obj.make_sound()` 流程

以 `let obj: &dyn Animal = &cat` 为例：

1. 从胖指针取 **`data_ptr`**、**`vtable_ptr`**
2. 在 vtable 中查 **`make_sound`** 对应槽位的 **fn 指针**
3. 以 **`data_ptr` 作 `&self`** 调用该 fn
4. 实际执行 **`Cat::make_sound(&cat)`**（或 `Dog::…`）

→ 比静态分发多 **vtable 间接**；优化器通常**难 inline** → [05 编译与分发](./05-compilation-dispatch.md)

### 3.5 对象安全（为何有些 trait 不能 `dyn`）

vtable 需要**固定数量、固定签名**的 fn 指针槽位：

| 不允许 | 原因 |
|--------|------|
| 泛型方法 `fn foo<T>(...)` | 无法为所有 `T` 列槽 |
| `fn new() -> Self` | `Self` 大小在 trait object 上未知 |
| 需要 `Self: Sized` 的方法 | `dyn Trait` **非 Sized** |

```rust
trait Animal { fn make_sound(&self); } // ✅ object-safe

trait Bad {
    fn foo<T>(&self, x: T); // ❌
    fn new() -> Self;       // ❌
}
```

关联类型 trait 作 `dyn` 时须 **指定关联类型**：`dyn Iterator<Item = u32>` → [06 泛型 Trait](./06-generic-traits.md)

### 3.6 验证大小（注意 API 区别）

```rust
use std::mem::{size_of, size_of_val};

let obj: &dyn Animal = &cat;

size_of::<&dyn Animal>(); // 16 — 胖指针本身（64 位）
size_of_val(obj);         // Cat 的字节数 — 用 vtable 里的 size 查具体类型
// size_of_val(&obj)      // 8 — 这是「引用的引用」，不是 trait object 宽指针大小
```

→ [`layout-demo`](./layout-demo/) · `Animal` / `Cat` 示例

**unsafe 深潜**：Nomicon / rustc `VirtualPtr` — 仅调试；生产勿手搓 vtable。

---

## 四、与 `Vec` / 容器

| | 栈上句柄 | 堆上 payload |
|---|----------|--------------|
| **`Vec<T>`** | `{ ptr, len, cap }` — 24B（64 位） | 连续 `T` |
| **`&[T]`** | 宽指针 `{ ptr, len }` — 16B | 不拥有，只借用 |

`Vec` 的 `{ ptr, len, cap }` 可看作 **拥有型** slice + 容量；设计思路上也是 **数据指针 + 元数据**。

→ [第 1 章 03.1 栈/堆](../Chapter-01-Foundations/03-1-rust-memory-model.md)

---

## 五、HFT / FFI 实战要点

### 1. 性能：`dyn Trait` 与 vtable

| | 影响 |
|---|------|
| **胖指针** | 多 8B 元数据 + 读 vtable 指针 |
| **方法调用** | **两次内存相关访问**（vtable → fn ptr）+ **间接跳转** |
| **分支预测** | 目标地址运行时才定，热路径可能更差 |
| **静态 vs 动态** | 静态：代码膨胀、运行最快；`dyn`：一份 vtable、运行多间接 |

纳秒级路径：少 `dyn`、少额外 indirection；必要时用具体类型或函数指针表（可控 layout）→ [05 编译与分发](./05-compilation-dispatch.md)

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

## 六、与分发 / 存在类型

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
