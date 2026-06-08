# Item 19: Avoid reflection

> **Effective Rust** · [Chapter 3 — Concepts](../../ER-本书目录.md)  
> **中文**：避免使用反射  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `dyn Trait`、虚表 | [17.2 trait 对象](../../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| DST、`?Sized` | [19.3 高级类型](../../../Book/19-advanced-features/19.3-高级类型.md) |
| 泛型 vs trait 对象 | [Item 12](../../Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md) |
| 派生宏代替运行时内省 | [Item 28](../../Chapter-05-Tooling/Item-28-macros-judiciously/README.md)（待补） |
| 标记 trait | [Item 10](../../Chapter-02-Traits/Item-10-standard-traits/README.md) |

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
其他语言：反射驱动架构（注册表、插件、ORM…）
         ↓
Rust：编译期单态化 + trait + 宏
         ↓
type_name：编译期泛型 → 对 &dyn Draw 只能看到 Draw，不是 Square
Any：      blanket impl 要求 T: 'static + ?Sized
         ↓
生命周期运行时擦除 → 禁止带非 'static 借用的 Any（防类型混淆）
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 19](../../ER-拓展索引.md#item-19)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
