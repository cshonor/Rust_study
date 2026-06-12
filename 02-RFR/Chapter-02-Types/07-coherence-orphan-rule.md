# 2.3 Coherence and the Orphan Rule（相干性与孤儿规则）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

为保证「某个 `(Type, Trait)` 组合在全局**至多一个** impl」，Rust 限制 `impl Trait for Type` 的落点。

## 孤儿规则 (Orphan Rule)

**典型表述**：`impl` 块中 **Trait 与 Type 至少其一由当前 crate 定义**。

- 不能在外部 crate 为 `(ForeignType, ForeignTrait)` 直接写 impl。
- 防止两个库各自为同一组合实现，导致冲突与不可预测行为。

## 覆盖规则 (Coverage)

- 在本地类型出现在合适位置时，可为外部类型实现外部 Trait 的部分模式（如 `impl From<MyType> for Foreign`）。
- **blanket impl**（`impl<T: Debug> MyTrait for T`）与 coherence 的交互需小心—— 过度宽泛的 blanket 可能与未来 impl 冲突。

## 实践

- 扩展外部类型：优先 **newtype** → [ER Item 06](../../01-ER/Chapter-01-Types/Item-06-newtype-pattern/README.md)
- 为外部错误类型加 context：本地 wrapper + `From` / `thiserror`

Book → [10.2 trait · 孤儿规则](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md)
