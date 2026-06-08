# Item 9: Consider using iterator transforms instead of explicit loops

> **Effective Rust** · [Chapter 1 — Types](../../ER-本书目录.md)  
> **中文**：考虑用迭代器转换替代显式循环  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 闭包（迭代器里常用） | [13.1 闭包](../../../Book/13-iterators-closures/13.1-闭包.md) |
| `Iterator`、`map`/`filter`/`collect` | [13.2 迭代器](../../../Book/13-iterators-closures/13.2-使用迭代器处理元素序列.md) |
| 循环 vs 迭代器性能 | [13.4 性能对比](../../../Book/13-iterators-closures/13.4-性能对比-循环-vs-迭代器.md) |
| `Option`/`Result` 链式 | [Item 3](../Item-03-option-result-transforms/README.md) |
| 控制流 `for` | [3.5 控制流](../../../Book/03-common-concepts/3.5-控制流.md) |

---

## 一句话

**优先迭代器链**

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
while → 下标 for → for-each → 迭代器链（源 + 适配器 + 消费者）
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 09](../../ER-拓展索引.md#item-09)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
