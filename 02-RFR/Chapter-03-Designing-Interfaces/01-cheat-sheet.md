# 1.1 · 命名惯例 · 速记

← [01 Naming Practices](./01-naming-practices.md) · [章索引](./README.md)

---

## 五系列（01-1 ~ 01-5）

| 前缀 | 口诀 | 详例 |
|------|------|------|
| `as_` | 只借，不消耗 | [01-1](./01-1-as-series.md) |
| `into_` | 交出自己 | [01-2](./01-2-into-series.md) |
| `get_` | 安全 `Option` | [01-3](./01-3-get-series.md) |
| `try_` | 失败 `Result` | [01-4](./01-4-try-series.md) |
| `with_` | 构造 / 配置 | [01-5](./01-5-with-series.md) |

Demo → [naming-series-demo](./naming-series-demo/)

## 迭代器三巨头

| | 元素 | 容器 |
|---|------|------|
| `iter` | `&T` | 保留 |
| `iter_mut` | `&mut T` | 保留 |
| `into_iter` | `T` | 消耗 |

## for 自动选择

`for x in v` → `into_iter` · `&v` → `iter` · `&mut v` → `iter_mut`

## 自测

- [ ] 五个系列各自消耗还是借用 `self`？  
- [ ] `get(99)` 和 `[99]` 失败行为有何不同？
