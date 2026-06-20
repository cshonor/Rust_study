# 1.3 · 人体工程学 impl · 速记

← [03 Ergonomic Trait Implementations](./03-ergonomic-trait-implementations.md) · [章索引](./README.md)

---

## 两种形态

A 条件 impl on T · B 包装转发 `&T`（§03 重点）

## 核心

blanket impl → `&T` / `&mut T` / Box/Arc 转发 · 仅**自定义 trait**

## 禁

全域 `impl<T: Debug> MyTrait for T` · 外部 trait 的 blanket

## 详例

→ [03-1 完整解读](./03-1-ergonomic-blanket-full-guide.md) · [demo](./blanket-trait-demo/)
