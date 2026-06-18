# 1.3 · 声明宏 macro_rules! · 速记

← [03 如何编写](./03-how-to-write-declarative-macros.md) · [02 如何工作](./02-how-declarative-macros-work.md) · [章索引](./README.md)

---

## 结构

```rust
macro_rules! name { (pat) => { … }; }
```

## 高频分类符

| 符 | 匹配 |
|----|------|
| `expr` | 表达式 |
| `ident` | 标识符 |
| `ty` | 类型 |
| `tt` | 任意 token 树（递归宏） |

## 重复语法

`$($x:expr),+` — `+` 至少 1 · `*` 可 0 · `?` 0 或 1

## 易错

具体模式在前 · 只有 `macro_rules!` = 声明宏 · `cargo expand` 调试

## 路线

expr/ident/ty → `$()*` vec! → `tt` 递归 → 复杂上过程宏

## 自测

- [ ] `$name:ident` 和 `$e:expr` 分别能匹配什么？  
- [ ] `sum!()` 空分支为何需要 `()` 规则？  
- [ ] 声明宏和 `sqlx::query!` 如何区分？
