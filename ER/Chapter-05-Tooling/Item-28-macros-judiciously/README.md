# Item 28: Use macros judiciously

> **Effective Rust** · [Chapter 5 — Tooling](../../ER-本书目录.md)  
> **中文**：明智地使用宏  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 声明宏 / 过程宏、`macro_rules!` | [19.5 宏](../../../Book/19-advanced-features/19.5-宏.md) |
| 泛型 vs 宏的抽象层级 | [10.1 泛型](../../../Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md)、[Item 12](../../Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md) |
| derive 代替运行时反射 | [Item 19](../../Chapter-03-Concepts/Item-19-avoid-reflection/README.md) |
| 过程宏供应链风险 | [Item 25](../../Chapter-04-Dependencies/Item-25-dependency-graph/README.md) |
| `cargo-expand` 等 | [Item 31](../Item-31-tooling-ecosystem/README.md)（待补） |

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
函数：     同类型不同值
泛型+trait：不同类型
宏：         同「语法角色」的不同程序片段（ident / expr / ty…）
         ↓
Rust 宏：Token / AST 级（非 C 文本替换）
         ↓
代价：难读、难调试、rustfmt/IDE 黑盒、编译期执行（Item 25）
         ↓
原则：函数/泛型不够再用；优先 derive
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 28](../../ER-拓展索引.md#item-28)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
