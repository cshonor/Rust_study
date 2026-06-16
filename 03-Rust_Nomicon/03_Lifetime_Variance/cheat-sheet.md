# 速记 · Ownership & Lifetimes · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **引用不能比 referent 活得久；`&mut` 不能有别名。**
2. **`&mut T` 对 T 不变；函数参数是逆变主来源。**
3. **不相交借用须 unsafe 证明 → `split_at_mut` 模式。**

## 自测

- [ ] 能解释协变/逆变/不变各一例
- [ ] 能说明何时需要 `PhantomData`
- [ ] 能对照 [split_borrows.rs](./src/split_borrows.rs) 说明为何需要 raw ptr
