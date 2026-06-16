# 2.3 Coherence and the Orphan Rule（相干性与孤儿规则）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

前置 → [06 泛型 Trait](./06-generic-traits.md) · 下一节 → [08 Trait 限定](./08-trait-bounds.md)

---

Rust 强制限制 `impl Trait for Type` 的编写位置，保证全局 `(类型, trait)` 组合**至多一份** impl，消除编译冲突与不可预测行为。

---

## 一、核心目标

杜绝多个 crate 为同一组（外部类型 + 外部 Trait）各自实现，避免：

1. **编译期歧义** — 编译器不知选哪份 impl  
2. **运行时不可预测** — 若允许链接期随机选 impl，行为无法保证  

这套规则统称 **coherence（相干性）**；其中对 impl 落点最核心的约束叫 **孤儿规则（Orphan Rule）**。

---

## 二、孤儿规则 Orphan Rule

### 规则原文

实现 `trait for 类型` 时，**Trait 和类型二者至少有一个是当前 crate 内定义**。

### 两条关键约束

1. **禁止场景（双外部）**  
   当前 crate 既没定义该 Type、也没定义该 Trait，不能写 `impl ForeignTrait for ForeignType`。  
   例：标准库 `Vec`（外部类型）+ `serde::Serialize`（外部 Trait），你的项目里不能直接 `impl Serialize for Vec<u8>`。

2. **设计初衷**  
   若放开限制，库 A、库 B 都给 `Vec` 实现 `Serialize`，项目同时依赖 A、B 时编译器无法区分该选用哪一份 impl → **E0117** 或一致性冲突。

### 合法 / 非法对照

```rust
// ❌ 非法：Vec、外部 Trait 均来自外部 crate
// impl std::fmt::Display for Vec<u8> {}

// ✅ 合法 1：Trait 本地，类型外部
trait MySerialize {
    fn tag(&self) -> &'static str;
}
impl MySerialize for Vec<u8> {
    fn tag(&self) -> &'static str { "vec-u8" }
}

// ✅ 合法 2：类型本地，Trait 外部
struct MyVec(Vec<u8>);
impl std::fmt::Display for MyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyVec({:?})", self.0)
    }
}
```

→ 可运行示例：[orphan-rule-demo](./orphan-rule-demo/)

---

## 三、覆盖规则 Coverage（孤儿规则例外）

满足条件时，允许给**外部类型**实现**外部 Trait**，核心是**本地类型出现在泛型参数等「本地元素」位置**。

### 合法示例

```rust
struct MyData;

// From 是外部 trait，Vec<u8> 是外部类型，但泛型参数 MyData 是本地类型
impl From<MyData> for Vec<u8> {
    fn from(_: MyData) -> Self {
        vec![1, 2, 3]
    }
}
```

拆解：

| 成分 | 来源 |
|------|------|
| Trait `From` | 外部（标准库） |
| 目标 Type `Vec<u8>` | 外部 |
| 泛型参数 `MyData` | **本地** → 满足孤儿规则 |

### 风险点：Blanket Impl（泛型全覆盖实现）

```rust
trait MyLocalTrait {}
impl<T: std::fmt::Debug> MyLocalTrait for T {}
```

1. **合法前提**：`MyLocalTrait` 是你本地定义的 trait。  
2. **潜在隐患**：过度宽泛的 blanket 会**抢占**未来其它 impl，触发 **coherence error**。

举例：后续第三方为 `Vec<u8>` 实现 `MyLocalTrait`，但你的 blanket 已覆盖所有 `T: Debug`（包含 `Vec<u8>`）→ 与「至多一个 impl」冲突。

**工程建议**：blanket 尽量加**紧**的 bound，或配合 **sealed trait** 限制实现范围 → [Ch03 §12](../Chapter-03-Designing-Interfaces/12-trait-implementations.md)。

---

## 四、工程实践

### 场景：给纯外部类型实现外部 Trait（违反基础孤儿规则）

优先 **NewType 包装模式**：

1. 用元组结构体包裹外部类型，定义**本地新类型**：

```rust
pub struct WrapperVec(Vec<u8>);
```

2. 针对本地 `WrapperVec` 实现外部 Trait — 类型是本地，完全符合孤儿规则。  
3. 通过 `Deref` / `AsRef` 透明访问内部 `Vec`，不丢失原有 API。

```rust
use std::ops::Deref;

impl Deref for WrapperVec {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for WrapperVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WrapperVec({})", self.0.len())
    }
}
```

### 三条捷径（何时不必 NewType）

| 你拥有 | 可做 |
|--------|------|
| **本地 Trait** | 任意给外部类型 `impl MyTrait for ForeignType` |
| **本地 Type** | 任意 `impl ForeignTrait for MyType` |
| **双外部** | **必须** NewType（或 fork / 上游 PR） |

### 常见场景

- 为外部错误类型加 context：本地 wrapper + `From` / `thiserror` → [ER Item 04](../../01-ER/Chapter-01-Types/Item-04-idiomatic-error-types/README.md)  
- 扩展 Iterator：本地 `trait MyExt` + blanket `impl<T: Iterator> MyExt for T` → [Ch13 extension traits](../Chapter-13-Rust-Ecosystem/07-extension-traits.md)

---

## 五、常见误区

| 误区 | 纠正 |
|------|------|
| 「孤儿规则禁止 impl 外部类型」 | 只禁止**双外部**；本地 trait 或本地 type 即可 |
| 「NewType 有运行时开销」 | 元组 newtype **零成本**抽象；`#[repr(transparent)]` 可保证 ABI |
| 「Coverage 可以随便 impl 外部 for 外部」 | 须有**本地类型**出现在规则允许的位置 |
| 「blanket 越多越好」 | 过宽会与未来 impl / 下游 crate 冲突 |
| 「sealed 能绕过孤儿」 | sealed 限制**谁**能 impl，**不**替代孤儿规则 |

---

## 六、速记

1. **双外部不能 impl；本地 trait 或本地 type 至少一个。**  
2. **Coverage：本地类型进泛型参数 → `impl From<My> for Foreign`。**  
3. **双外部要扩展 → NewType + `Deref`；blanket 别铺太宽。**

---

## 对照阅读

- Book → [10.2 trait · 孤儿规则](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md) · [19.2 newtype](../../00-Book/19-advanced-features/19.2-高级trait.md)
- ER → [Item 06 Newtype](../../01-ER/Chapter-01-Types/Item-06-newtype-pattern/README.md)
- Ch03 → [04 Wrapper Types](../Chapter-03-Designing-Interfaces/04-wrapper-types.md) · [12 Trait 实现](../Chapter-03-Designing-Interfaces/12-trait-implementations.md)
- Demo → [orphan-rule-demo](./orphan-rule-demo/)

```bash
cd 02-RFR/Chapter-02-Types/orphan-rule-demo
cargo run
```
