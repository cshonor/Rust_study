# 4.1 · 管理 Unsafe 边界 · 速记

← [10 Manage Boundaries](./10-manage-boundaries.md) · [11 文档](./11-documentation.md) · [章索引](./README.md)

---

## 三大准则

1. **缩小边界** — private 字段 + safe 对外 API + 极小 `unsafe {}`  
2. **同一信任域** — 改不变量的代码全在一个 mod/impl 内  
3. **优先安全替代** — 标准库 / 成熟 crate；能不写 unsafe 就不写  

## 取舍流程

Safe 重构 → 标准库 → 社区 crate → 最后才手写 unsafe

## 不变量

指针有效 · len/cap · 初始化/Validity · 所有权 — **不得泄露到模块外**

## 配套

`// SAFETY:` · Miri · ER Item 16

## 自测

- [ ] 为何 `pub ptr: *mut T` 会破坏边界抽象？  
- [ ] 「信任域」与「缩小边界」差在哪？  
- [ ] 何时才值得手写 unsafe？
