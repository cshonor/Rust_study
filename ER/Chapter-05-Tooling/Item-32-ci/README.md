# Item 32: Set up a continuous integration (CI) system

> **Effective Rust** · [Chapter 5 — Tooling](../../ER-本书目录.md)  
> **中文**：建立持续集成 (CI) 系统  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [CI 示例](../../.github/workflows/er-study-ci.yml)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `cargo test`、测试组织 | [11.1](../../../Book/11-testing/11.1-如何编写测试.md)、[11.3](../../../Book/11-testing/11.3-测试的组织结构.md) |
| `Cargo.lock`（app vs lib） | [Item 25](../../Chapter-04-Dependencies/Item-25-dependency-graph/README.md) |
| 工具清单 | [Item 31](../Item-31-tooling-ecosystem/README.md) |

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
ER 全书建议（Clippy、deny、features、doc…）
         ↓
仅文档 / 口头 → 很快腐化
         ↓
CI 自动化 → 真正防线
         ↓
流程 Bug（忘跑 codegen）→ 先加 CI 步骤再修（同 Item 30 TDD）
         ↓
全绿铁律 + 本地可复现 → 人不替机器背锅
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 32](../../ER-拓展索引.md#item-32)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
