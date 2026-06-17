# 3 · 存在类型 · 速记

← [10 hub](./10-existential-types.md) · [05 分发速记](./05-cheat-sheet.md) · [章索引](./README.md)

---

## 三句话

1. **`impl Trait` = ∃T：存在某类型，对外匿名，编译期定死。**  
2. **返回位藏长类型；参数位 = 匿名 `<T: Trait>`。**  
3. **要异构集合 → `dyn`；要同参同型 → 命名 `T`。**

## 自测

- [ ] `impl Trait` 与 `dyn Trait` 分发机制差在哪？  
- [ ] 为何 `if/else` 不能返回两种不同迭代器（裸 `impl Iterator`）？  
- [ ] `∃` 与 `for<'a>`（`∀`）各对应什么语法？
