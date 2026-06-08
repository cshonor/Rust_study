# Item 3: Prefer Option and Result transforms over explicit match

> **Effective Rust** · [Chapter 1 — Types](../../ER-本书目录.md)  
> **中文**：优先使用 Option / Result 的转换方法，而非显式 `match`  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-03-option-result](./demo/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `Option`、`match` / `if let` | [6.1 定义枚举](../../../Book/06-enums-pattern-matching/6.1-定义枚举.md)、[6.3 if let](../../../Book/06-enums-pattern-matching/6.3-if-let.md) |
| `Result`、`?` | [9.2 Result](../../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |
| panic / unwrap | [9.1 panic](../../../Book/09-error-handling/9.1-panic-与不可恢复的错误.md)、[9.3 策略](../../../Book/09-error-handling/9.3-使用panic还是不用panic.md) |
| 错误类型设计 | [Item 4](../Item-04-idiomatic-error-types/README.md)（ER） |

---

## 一句话

**少写无意义的 `match`**

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
冗长 match
  → if let（只关心一边）
  → map / and_then / map_err 等（函数式链）
  → ?（错误向上传递，最少样板）
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 03](../../ER-拓展索引.md#item-03)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
