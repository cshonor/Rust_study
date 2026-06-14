# 2.2 Generic Traits（泛型 Trait）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

Trait 里描述「类型关系」有两种核心机制：**关联类型** vs **泛型 trait 参数** — 适用场景和设计直觉完全不同。

前置 → [05 编译与分发](./05-compilation-dispatch.md) · 存在类型 → [10 impl Trait](./10-existential-types.md)

---

## 一、对照总表

| 机制 | 同一 `(Type, Trait)` 能有几份「选择」 | 典型 |
|------|--------------------------------------|------|
| **关联类型 (Associated Type)** | 通常 **只有一种** | `Iterator::Item` |
| **泛型 trait 参数** | 可对 **不同类型多次 impl** | `PartialEq<Rhs>`、`From<T>` |

---

## 二、关联类型：「一对一」

**问的是**：*这个类型实现该 trait 时，**对应的**类型是什么？* — 一种 impl 里**唯一确定**。

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for std::vec::IntoIter<i32> {
    type Item = i32; // 对这份 impl，Item 只能是 i32
    fn next(&mut self) -> Option<Self::Item> { /* ... */ }
}
```

| 特点 | |
|------|---|
| 每个 `(Self, Trait)` **一份**关联类型绑定 | 签名简洁：`Iterator` 而非 `Iterator<Item>` |
| 适合描述**固有属性** | 元素类型、Error 类型、Output 等 |
| 与 `dyn` | 作 trait object 时须 **指定关联类型**（见下） |

**HFT 直觉**：订单簿 iterator 的 `Item = OrderId` — 一种结构一种元素，用关联类型表达最清楚。

---

## 三、泛型 trait 参数：「多对多」

**问的是**：*这个 trait 可以和**哪些其它类型**交互？* — 同一 `Self` 可 **多次 impl**。

```rust
trait PartialEq<Rhs = Self> {
    fn eq(&self, other: &Rhs) -> bool;
}

impl PartialEq<u32> for u32 { /* u32 == u32 */ }
impl PartialEq<u64> for u32 { /* u32 == u64，若你实现 */ }
```

```rust
trait From<T> {
    fn from(value: T) -> Self;
}
// 同一 Target 可对多个 T：From<u32>、From<String> …
```

| 特点 | |
|------|---|
| **灵活** — 多种 RHS / 源类型 | 签名更复杂：`PartialEq<Rhs>` |
| 适合 **跨类型** 比较、转换、运算 | `Add<Rhs>`、`TryFrom<T>` 等 |
| monomorph | 每种 `(Self, Trait, Rhs)` 组合可生成一份代码 → 注意**膨胀** |

---

## 四、怎么选（设计直觉）

| 问题 | 倾向 |
|------|------|
| 「这个类型的 **Item/Error/Output** 是什么？」 | **关联类型** |
| 「能和 **哪些其它类型** 比/转/算？」 | **泛型参数** |
| 希望 trait 名简短、impl 块内唯一 | **关联类型** |
| 需要同一 type 上多份不同 RHS 行为 | **泛型参数** |

**默认倾向**：能写成关联类型就写关联类型；只有确实需要 **多 impl** 再用泛型 trait。

---

## 五、与存在类型 / trait object

| 写法 | 分发 | 关联类型 |
|------|------|----------|
| **`impl Iterator<Item = u32>`** | **静态** — 隐藏具体迭代器类型 | 在 bound 里 **指定** `Item` |
| **`dyn Iterator<Item = u32>`** | **动态** — vtable + 宽指针 | **必须**写清 `Item`（及其他 GAT 若适用） |
| **`impl Trait` 返回** | 静态，HFT 热路径友好 | 见 [10](./10-existential-types.md) |

```rust
fn count_u32(it: impl Iterator<Item = u32>) -> usize { it.count() }

fn count_dyn(it: &mut dyn Iterator<Item = u32>) -> usize { it.count() } // 动态
```

带 **未指定关联类型的 `dyn Iterator`** 通常 ❌ — 编译器不知 `Item` 大小与 vtable 布局。

→ [04 DST / 宽指针](./04-dst-wide-pointers.md) · [05 静态 vs 动态](./05-compilation-dispatch.md)

---

## 六、HFT 实战要点

### 1. 固有属性 → 关联类型

行情解析器、订单流：

```rust
trait TickSource {
    type Tick; // 固定一种 Tick 结构
    fn next(&mut self) -> Option<Self::Tick>;
}
```

意图清晰；少一层 trait 泛型参数；利于 monomorph inline。

### 2. 跨类型交互 → 泛型 trait

```rust
trait FromWire<T> {
    fn decode(raw: T) -> Self;
}
// 或标准库 From<T> / TryFrom<T>
```

不同 wire 格式、不同 `T` — 才用泛型 trait 参数。

### 3. 警惕泛型 trait 导致代码膨胀

每个 `(Self, Trait, Rhs)` 组合可能 **单独 monomorph** → 编译时间与二进制 ↑。HFT 里 trait 设计宜 **少而精**。

### 4. 核心路径：`impl Trait`，慎用 `dyn`

- 返回迭代器 / 策略组件：`-> impl Iterator<Item = Tick>`  
- 插件边界才 `Box<dyn ...>`，且写全 `Item = ...`

### 5. GAT（了解）

Rust 的 **Generic Associated Types** 把关联类型也参数化 — 高级场景（如 lending iterator）。日常 HFT 先掌握 **普通关联类型 vs 泛型 trait** 即可。

---

## 易错点

| 易错 | 纠正 |
|------|------|
| 凡事 `Trait<Rhs>` | 固有 `Item` 用 **关联类型** |
| `dyn Iterator` 不写 `Item` | 须 **`dyn Iterator<Item = T>`** |
| 关联类型 = 性能差 | 与泛型 trait **都能静态分发**；差异在 **能 impl 几份** |
| `impl Iterator` 和 `dyn Iterator` 一样快 | **`impl` 静态**，`dyn` 有 vtable |

---

## 对照阅读

- Book → [10.2 trait](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md)
- ER → [Item 13 默认实现](../../01-ER/Chapter-02-Traits/Item-13-default-implementations/README.md) · [Item 12 分发](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md)
- 下一节 → [07 相干性与孤儿规则](./07-coherence-orphan-rule.md)
