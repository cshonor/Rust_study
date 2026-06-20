# 1.3 Ergonomic Trait Implementations（人体工程学 impl）

> 所属：**Unsurprising** · [← 章索引](./README.md)

← [02 通用 Trait](./02-common-traits-for-types.md) · 下一节 [04 包装类型](./04-wrapper-types.md)

**目标**：消除调用分叉 — 让 `&T`、`Box<T>` 等也能直接调用**你定义的 trait** 方法。

---

## 核心：Blanket impl（两种形态）

| 形态 | 写法 | 作用 |
|------|------|------|
| **A 条件补能力** | `impl<T: Other> MyTrait for T` | 满足条件的 **T** 自动获得 trait |
| **B 包装转发** | `impl<T: MyTrait> MyTrait for &T` | **引用/指针**也能调，消除 `*` 分叉 |

§03 重点在 **形态 B**；形态 A 绑 `Debug` 等过宽 trait 易冲突。

```rust
trait MyTrait { fn work(&self); }

impl<T: MyTrait + ?Sized> MyTrait for &T {
    fn work(&self) { (*self).work() }
}
```

---

## 约束（两线）

| 约束 | 要点 |
|------|------|
| **孤儿 / 相干** | 仅**本 crate 自定义 trait**；不能给 `Debug` 等外部 trait 写 blanket |
| **过宽 blanket** | `impl<T: Debug> MyTrait for T` 易与未来 impl 冲突 → 收窄 bound 或 [sealed trait](./12-trait-implementations.md) |

---

→ **完整解读 + 可复制模板 + demo**：[03-1 人体工程学 Blanket 完整解读](./03-1-ergonomic-blanket-full-guide.md) · [03-1 速记](./03-1-cheat-sheet.md) · [03 速记](./03-cheat-sheet.md)

ER → [Item 13 默认实现](../../01-ER/Chapter-02-Traits/Item-13-default-implementations/README.md) · 相干 → [07 孤儿规则](../Chapter-02-Types/07-coherence-orphan-rule.md)
