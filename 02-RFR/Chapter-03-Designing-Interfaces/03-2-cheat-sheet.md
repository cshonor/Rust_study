# ?Sized · 速记

← [03-2 详解](./03-2-question-sized.md) · [03-1 Blanket](./03-1-ergonomic-blanket-full-guide.md)

---

## `?`

否定默认隐式 bound · 不是加新条件

## 默认

泛型 `T` 隐式 `T: Sized`

## `T: ?Sized`

T 可为 DST（str / [T] / dyn Trait）

## blanket

约束在 **T** 上 · `&str` 要 `T=str` 能 !Sized → 须 `?Sized`

## 无关

§02 三类 trait · default method · 官方/自定义 impl 均同

## 自测

- [ ] 不加 `?Sized` 时 `T=str` 能匹配 `impl for &T` 吗？
