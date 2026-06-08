# Item 22: Minimize visibility

> **Effective Rust** · [Chapter 4 — Dependencies](../../ER-本书目录.md)  
> **中文**：最小化可见性  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-22-visibility](./demo/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 模块、默认私有、`pub` | [7.2 定义模块与私有性](../../../Book/07-packages-modules/7.2-定义模块来控制作用域与私有性.md) |
| `pub struct` 与字段 | [7.3 路径](../../../Book/07-packages-modules/7.3-路径用于引用模块树中的项.md) |
| 模块分文件 | [7.5 模块分割进不同文件](../../../Book/07-packages-modules/7.5-将模块分割进不同文件.md) |
| `pub use` 对外 API | [14.2 发布 crate](../../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| Semver 与 API 变更 | [Item 21](../Item-21-semver/README.md) |
| 重导出门面 | [Item 24](../Item-24-re-export-api-types/README.md)（待补） |

---

## 一句话

见 [03-key-takeaways.md](./topics/03-key-takeaways.md)。

---

## 专项笔记（按需点开）

| # | 专题 | 阅读 |
|---|------|------|
| 01 | 核心知识点 | [topics/01-core-concepts.md](./topics/01-core-concepts.md) |
| 02 | 逻辑脉络 | [topics/02-logic-flow.md](./topics/02-logic-flow.md) |
| 03 | 重点结论 | [topics/03-key-takeaways.md](./topics/03-key-takeaways.md) |
| 04 | 案例与代码 | [topics/04-examples.md](./topics/04-examples.md) |
| 05 | 易错细节 | [topics/05-pitfalls.md](./topics/05-pitfalls.md) |
| — | 背诵提纲 | [topics/cheat-sheet.md](./topics/cheat-sheet.md) |

---

## 逻辑脉络

```text
默认私有 → 只暴露必要 API
         ↓
pub 扩大可见性 = 消耗未来重构自由（Semver 单向门）
  私有 → 公开：Minor（兼容）
  公开 → 私有：Major（break）
         ↓
字段/内部类型公开 → 换实现 = Major
隐藏细节 → 内部可优化而不 break 用户
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 22](../../ER-拓展索引.md#item-22)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
