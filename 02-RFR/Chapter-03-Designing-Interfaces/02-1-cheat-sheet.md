# 通用 Trait · 速记

← [02-1 完整解读](./02-1-common-traits-full-guide.md) · [02 通用 Trait](./02-common-traits-for-types.md)

---

## 三类

| 类 | Trait |
|----|-------|
| 几乎总有 | `Debug` · `PartialEq` / `Eq` |
| 线程假设 | `Send` · `Sync`（多数自动） |
| 谨慎 | `Copy` · `Hash` |

## 默认 derive

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Widget { /* … */ }
```

## 作 map 键

+ `Hash` · 相等与哈希一致 · 入集合后勿改键字段

## Copy

仅小栈值 · 有 `String`/`Vec` 就别 Copy

## Send 例外

`Rc` 非 Send → 文档 + `Arc` 替代

## 模板

A 通用 · B +Hash 键 · C Copy 小值 · D 手动 Debug · E 非 Send 说明

## 自测

- [ ] `f64` 能 derive `Eq` 吗？  
- [ ] `HashMap` 键要哪几个 trait？  
- [ ] 给 struct 加了 `Copy` 后还能加 `String` 字段吗？
