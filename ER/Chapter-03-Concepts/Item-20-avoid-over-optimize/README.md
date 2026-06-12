# Item 20: Avoid the temptation to over-optimize

> **Effective Rust** · [Chapter 3 — Concepts](../../ER-本书目录.md)  
> **中文**：避免过度优化的诱惑  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-20-tlv](./demo/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 所有权、显式拷贝 | [4.1 什么是所有权](../../../Book/04-ownership/4.1-什么是所有权.md) |
| 生命周期传染 | [Item 14](../Item-14-lifetimes/README.md)、[10.3 专题](../../../Book/10-generics-traits-lifetimes/10.3-生命周期与引用有效性.md) |
| 智能指针 | [Item 8](../../Chapter-01-Types/Item-08-references-pointers/README.md)、[15 章](../../../Book/15-smart-pointers/) |
| `Copy` / 标准 trait | [Item 10](../../Chapter-02-Traits/Item-10-standard-traits/README.md) |
| 迭代器 vs 循环（先测再信） | [13.4 性能对比](../../../Book/13-iterators-closures/13.4-性能对比-循环-vs-迭代器.md) |
| `no_std` 硬约束 | [Item 33](../../Chapter-06-Beyond-Standard-Rust/Item-33-no-std/README.md)（待补） |
| `cargo bench` | [Item 31](../../Chapter-05-Tooling/Item-31-tooling-ecosystem/README.md)（待补） |

---

## 一句话

见 [03-key-takeaways.md](./03-key-takeaways.md)。

---

## 专项笔记（按需点开）

| # | 专题 | 阅读 |
|---|------|------|
| 01 | 核心知识点 | [01-core-concepts.md](./01-core-concepts.md) |
| 02 | 逻辑脉络 | [02-logic-flow.md](./02-logic-flow.md) |
| 03 | 重点结论 | [03-key-takeaways.md](./03-key-takeaways.md) |
| 04 | 案例与代码 | [04-examples.md](./04-examples.md) |
| 05 | 易错细节 | [05-pitfalls.md](./05-pitfalls.md) |
| — | 背诵提纲 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

```text
零拷贝：struct Tlv<'a> { value: &'a [u8] }  → 解析快
         ↓
放进长生命周期状态：NetworkServer 缓存 Tlv<'a>
         ↓
数据来源是循环里临时 Vec → data does not live long enough
         ↓
破局：struct Tlv { value: Vec<u8> } + .to_vec()
         ↓
一次分配换掉<'a>，代码恢复灵活
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 20](../../ER-拓展索引.md#item-20)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
