# 2.2 Generic Traits（泛型 Trait）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

## 关联类型 vs 泛型参数

| 机制 | 同一 `(Type, Trait)` 能有几份「选择」 | 典型 |
|------|--------------------------------------|------|
| **关联类型** | 通常 **一种** | `Iterator::Item` |
| **泛型 Trait 参数** | 可对 **不同 RHS** 多次 impl | `PartialEq<Rhs>`、`From<T>` |

## 设计直觉

- **关联类型**：「这个类型作为 `Iterator` 时，元素类型**就是** X」—— impl 块内唯一。
- **泛型参数**：「可以与多种类型比较 / 转换」—— 多 impl 共存。

## 与存在类型的对比

- `impl Iterator<Item = u32>` 隐藏具体迭代器类型 → [10 存在类型](./10-existential-types.md)
- `dyn Iterator<Item = u32>` 动态分发 + DST

Book → [10.2 trait](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md) · ER → [Item 13 默认实现](../../01-ER/Chapter-02-Traits/Item-13-default-implementations/README.md)
