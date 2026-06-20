# 1.1 · 命名惯例 · 速记

← [01 Naming Practices](./01-naming-practices.md) · [章索引](./README.md)

---

## 六系列口诀

| 前缀 | 口诀 |
|------|------|
| `as_` | 只借，不消耗 |
| `into_` | 交出自己 |
| `get_` | 安全 `Option`，不 panic |
| `try_` | 失败 `Result` |
| `with_` | 构造 / 配置 → [01-1 详例](./01-1-with-series-and-builder.md) |

## 迭代器三巨头

| | 元素 | 容器 |
|---|------|------|
| `iter` | `&T` | 保留 |
| `iter_mut` | `&mut T` | 保留 |
| `into_iter` | `T` | 消耗 |

## for 自动选择

`for x in v` → `into_iter` · `&v` → `iter` · `&mut v` → `iter_mut`

## 自测

- [ ] `Some(x).as_ref()` 后原 `Option` 还能用吗？  
- [ ] `for x in v` 和 `for x in &v` 对所有权差在哪？  
- [ ] `get(99)` 和 `[99]` 失败行为有何不同？
