# 2.3.3 · NewType 与工程实践

> 所属：**Traits and Trait Bounds · 相干性** · [← 07 hub](./07-coherence-orphan-rule.md)

← [07.2 Coverage 与 Blanket](./07-2-coverage-blanket.md) · 下一节 [08 Trait 限定](./08-trait-bounds.md)

---

## 一、工程实践：双外部时的 NewType

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

## 二、常见误区

| 误区 | 纠正 |
|------|------|
| 「孤儿规则禁止 impl 外部类型」 | 只禁止**双外部**；本地 trait 或本地 type 即可 |
| 「NewType 有运行时开销」 | 元组 newtype **零成本**抽象；`#[repr(transparent)]` 可保证 ABI |
| 「Coverage 可以随便 impl 外部 for 外部」 | 须有**本地类型**出现在规则允许的位置 |
| 「blanket 越多越好」 | 过宽会与未来 impl / 下游 crate 冲突 |
| 「sealed 能绕过孤儿」 | sealed 限制**谁**能 impl，**不**替代孤儿规则 |

---

## 三、速记

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

→ 下一节：[08 Trait 限定](./08-trait-bounds.md)
