# 人体工程学 blanket · 速记

← [03-1 完整解读](./03-1-ergonomic-blanket-full-guide.md) · [03 hub](./03-ergonomic-trait-implementations.md)

---

## 目标

引用 / 智能指针直接调 trait · 消除 `(*rf).work()` 分叉

## 模板

`impl<T: MyTrait + ?Sized> MyTrait for &T` · 配套 `&mut T` · 可选 Box/Arc/Rc

## 约束

仅**自己的 trait** · 禁全域 `impl<T: Debug> …` · 可 sealed

## 配套

ER Item 13 默认方法 · std 案例 ToString/Into

## 自测

- [ ] 能给 `std::fmt::Debug` 写 `impl for &T` 吗？  
- [ ] 过宽 blanket 未来冲突原因？
