# Item 13: Use default implementations to minimize required trait methods

> **Effective Rust** · [Chapter 2 — Traits](../../ER-本书目录.md)  
> **中文**：用默认实现最小化 trait 中必需的方法  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| trait 定义、默认方法 | [10.2 trait](../../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| `Iterator` / `next` | [13.2 迭代器](../../../Book/13-iterators-closures/13.2-使用迭代器处理元素序列.md) |
| 对象安全、`Sized` | [Item 12](../Item-12-generics-vs-trait-objects/README.md) |
| 迭代器适配器 | [Item 9](../../Chapter-01-Types/Item-09-iterator-transforms/README.md) |

---

## 一句话

**小强制面 + 大可用面**

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
强制少量基元方法（如 next / len）
  → trait 内默认实现派生大量便捷 API
  → 实现者省力、使用者功能全
  → 库演进：新增带默认体的方法 ≈ 向后兼容
  → 具体类型可 override 更高效实现
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 13](../../ER-拓展索引.md#item-13)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
