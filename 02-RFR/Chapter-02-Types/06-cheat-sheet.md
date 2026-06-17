# 2.2 · 泛型 Trait · 速记

← [06 hub](./06-generic-traits.md) · [章索引](./README.md)

---

## 对照

| | 关联类型 | 泛型 trait 参数 |
|---|----------|-----------------|
| 问什么 | 固有 Item/Error | 能和谁比/转 |
| impl 份数 | 通常 1 份 | 可多份 |
| 典型 | `Iterator::Item` | `PartialEq<Rhs>` |

## 三句话

1. **能关联类型就关联类型。**  
2. **`dyn Iterator` → 必须 `Item = T`。**  
3. **`impl` 静态，`dyn` vtable。**
