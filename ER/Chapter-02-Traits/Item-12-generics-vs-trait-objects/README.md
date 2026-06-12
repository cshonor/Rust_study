# Item 12: Understand generics vs trait objects trade-offs

> **Effective Rust** · [Chapter 2 — Traits](../../ER-本书目录.md)  
> **中文**：理解泛型与 trait 对象之间的权衡  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 泛型、trait bound | [10.1 泛型](../../../Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md)、[10.2 trait](../../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| `dyn Trait`、trait 对象 | [17.2 trait 对象](../../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| 胖指针 | [Item 8](../../Chapter-01-Types/Item-08-references-pointers/README.md) |
| 单态化、高级 trait | [19.2 高级 trait](../../../Book/19-advanced-features/19.2-高级trait.md) |
| 闭包 vs `fn` 静/动分发 | [Item 2](../../Chapter-01-Types/Item-02-express-common-behavior/README.md) |

---

## 一句话

**`stamp.make_copy()`**

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
泛型：编译期特化 → 快、可组合多 bound → 代码体积↑
dyn：  运行期 vtable → 异构集合、省代码 → 间接调用开销
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 12](../../ER-拓展索引.md#item-12)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
