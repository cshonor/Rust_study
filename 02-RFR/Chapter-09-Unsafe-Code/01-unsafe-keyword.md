# 1. The unsafe Keyword（unsafe 关键字）

> [← 章索引](./README.md)

## 双重角色

| 形式 | 含义 |
|------|------|
| **`unsafe fn`** | **声明契约** — 调用方须满足额外前提，否则 UB |
| **`unsafe { ... }`** | **履行契约** — 块内使用超能力，作者保证不变量 |

## 仍生效的规则

`&T` / `&mut T` 的借用检查在 `unsafe` 块中**仍然适用**。

`unsafe` 放开的是：裸指针解引用、调用 `unsafe fn`、`static mut`、`unsafe impl`、访问 `union` 等（与 Book 一致）。

Book → [19.1](../../00-Book/19-advanced-features/19.1-不安全Rust.md)
