# 3.5 The Drop Check（Drop 检查）

> 所属：**Great Responsibility** · [← 章索引](./README.md)

← [08 转换](./08-casting.md) · 下一节 [10 管理边界](./10-manage-boundaries.md)

编译器分析类型是否**安全地 drop**（无自引用、无未初始化字段被 drop 等）。

## 与 unsafe 的交界

- 手动 **`ManuallyDrop`** 跳过自动 drop
- **`forget`** / 泄漏 intentionally
- 自引用结构 + **`Pin`** — drop 顺序与移动约束

→ [第 1 章 · Drop 顺序](../Chapter-01-Foundations/04-ownership.md)

## 误用

在不变式破坏时仍触发 `Drop` → 与 [07 Panics](./07-panics.md) 同类风险。

Book → [15.3 Drop](../../00-Book/15-smart-pointers/15.3-使用Drop运行清理代码.md)
