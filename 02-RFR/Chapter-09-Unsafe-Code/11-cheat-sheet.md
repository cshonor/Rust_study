# 4.2 · 文档 · 速记

← [11 Documentation](./11-documentation.md) · [10 管理边界](./10-manage-boundaries.md) · [章索引](./README.md)

---

## 两套体系（不可混）

| | `/// # Safety` | `// SAFETY:` |
|---|----------------|--------------|
| 对象 | 对外 API 契约 | 对内局部证明 |
| 受众 | 调用者 | 维护者 |
| 定义/证明 | **定义**前置条件 | **证明**当前满足 |

## 行内 SAFETY 两层

1. 外部前置约束（指针/对齐/生命周期/竞争/所有权）  
2. 本地证明（上下文如何保证全部成立）

## ER Item 27 三区

- `# Safety` — unsafe 前置约束  
- `# Panics` — 安全函数 panic 边界  
- `# Invariants` — 结构体长期不变量  

## 三场景

1. safe 包装 → `# Invariants` + 内部 `// SAFETY:`  
2. `unsafe fn` → 完整 `# Safety` + 内部引用契约  
3. `unsafe trait/impl` → trait `# Safety` + impl 上 `// SAFETY:`  

## 硬性标准

一条 unsafe 一条注释 · 禁「this is safe」· 不变量集中写 struct 文档

## 自测

- [ ] 行内 SAFETY 与 rustdoc Safety 谁定义、谁证明？  
- [ ] safe 包装器为何要在 rustdoc 写 Invariants？  
- [ ] 为何不能多个 `unsafe {}` 共用一条 SAFETY？
