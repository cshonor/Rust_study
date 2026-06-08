# Item 4: Prefer idiomatic Error types

> **Effective Rust** · [Chapter 1 — Types](../../ER-本书目录.md)  
> **中文**：倾向使用符合惯例的 Error 类型  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-04-error-types](./demo/) · [item-04-core-error](./demo-core-error/)（no_std `Error`）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `Result`、`?` | [9.2 Result](../../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |
| `Display` / `Debug` | [10.2 trait](../../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| Newtype | [Item 6](../Item-06-newtype-pattern/README.md)（ER） |
| Option/Result 链式 | [Item 3](../Item-03-option-result-transforms/README.md)（ER） |

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
String 当错误（不行：无法 impl Error）
  → Newtype 包装（MyError(String)）
  → Enum 聚合子错误 + 自定义 source()
  → 库：具体 Enum + thiserror
  → 应用：Box<dyn Error> + anyhow
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 04](../../ER-拓展索引.md#item-04)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
