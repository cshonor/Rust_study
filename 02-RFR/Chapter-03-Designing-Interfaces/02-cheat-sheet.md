# 1.2 · 通用 Trait · 速记

← [02 Common Traits](./02-common-traits-for-types.md) · [章索引](./README.md)

---

## 三类策略

| | Trait |
|---|-------|
| 几乎总有 | `Debug` · `PartialEq` / `Eq` |
| 线程 | `Send` · `Sync` |
| 谨慎 | `Copy` · `Hash` |

→ 详例 + 可复制模板：[02-1 完整解读](./02-1-common-traits-full-guide.md)

## 自测

- [ ] 公开 struct 最少 derive 哪两个？  
- [ ] `Rc` 能 `thread::spawn` 吗？
