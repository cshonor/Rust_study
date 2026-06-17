# 4.2 Read and Write Documentation（文档）

> 所属：**Coping with Fear** · [← 章索引](./README.md)

← [10 管理边界](./10-manage-boundaries.md) · 下一节 [12 验证工作](./12-check-your-work.md)

每个 `unsafe` 块前用 **`// SAFETY:`**（或 crate 约定）写明：

- 调用者 / 维护者须保证的前提
- 为何当前代码满足这些前提

公开 safe API 包装 unsafe 时，在 rustdoc 写清 **Safety** / **Invariant** 段落。

ER → [Item 27 文档](../../01-ER/Chapter-05-Tooling/Item-27-document-public-api/README.md)
