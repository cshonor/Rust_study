# 通用 Trait · 速记

← [02-1 完整解读](./02-1-common-traits-full-guide.md) · [02 通用 Trait](./02-common-traits-for-types.md)

---

## 三类边界

| | 影响 | 代价 |
|---|------|------|
| Ⅰ `Debug`/`Eq` | 只增便利 | 不加难调试 |
| Ⅱ `Send`/`Sync` | 线程契约 | 非 Send 不能 spawn |
| Ⅲ `Copy`/`Hash` | 锁死/不变式 | 难演进 / map 失效 |

## derive

Debug·Eq ✅ · Copy 显式 · Hash+Eq 成对

## 默认 derive

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order { /* … */ }
```

## 模板

A 通用 · B +Hash 键 · C Copy 小值 · D 手动 Debug · E 非 Send 说明

## 自测

- [ ] 派生宏为何默认不给你 `Copy`？  
- [ ] 行情模块非 Send 会怎样？
