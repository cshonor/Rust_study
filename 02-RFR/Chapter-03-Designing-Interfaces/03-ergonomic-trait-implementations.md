# 1.3 Ergonomic Trait Implementations（人体工程学 impl）

> 所属：**Unsurprising** · [← 章索引](./README.md)

为自定义 Trait 减少「必须拥有 `T` 才能调用」的分叉。

## Blanket impl

对 **`&T`、`&mut T`、`Box<T>`、`Arc<T>`** 等做 blanket 实现，使用户在**仅持有引用或智能指针**时也能调用 Trait 方法。

```rust
trait MyTrait { fn work(&self); }

impl<T: MyTrait + ?Sized> MyTrait for &T {
    fn work(&self) { (*self).work() }
}
```

## 注意

- 与 [07 相干性与孤儿规则](../Chapter-02-Types/07-coherence-orphan-rule.md) 冲突时无法添加 blanket。
- 过宽的 blanket（`impl<T: Debug> MyTrait for T`）可能与未来 impl 打架—— 收窄 bound 或 sealed trait → [12 Trait 实现控制](./12-trait-implementations.md)。

ER → [Item 13 默认实现](../../01-ER/Chapter-02-Traits/Item-13-default-implementations/README.md)
