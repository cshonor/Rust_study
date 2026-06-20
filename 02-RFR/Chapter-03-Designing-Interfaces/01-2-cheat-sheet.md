# into_ · 速记

← [01-2](./01-2-into-series.md) · [01 命名 hub](./01-naming-practices.md)

---

## 口诀

交出自己 · 原变量失效

## 代表 API

`into_iter` · `into_inner`（包装器上，非 Guard）· `into_bytes`

→ `into_inner` 详例：[01-2-1](./01-2-1-into-inner.md) · std 无 `into_item`，用 `next()`

## 自测

- [ ] `into_iter` 后还能 `println!("{:?}", v)` 吗？
