# Item 2: Use the type system to express common behavior

> **Effective Rust** · [Chapter 1 — Types](../../ER-本书目录.md)  
> **中文**：使用类型系统表达公共行为  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [demo](./demo/) · `item-02-callbacks-generics`（四条结论可运行示例）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 函数、方法、`impl` | [3.3 函数](../../../Book/03-common-concepts/3.3-函数.md)、[5.3 方法语法](../../../Book/05-structs/5.3-方法语法.md) |
| 闭包、`Fn*` | [13.1 闭包](../../../Book/13-iterators-closures/13.1-闭包.md)、[19.4 高级函数与闭包](../../../Book/19-advanced-features/19.4-高级函数与闭包.md) |
| Trait、泛型 | [10.1 泛型](../../../Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md)、[10.2 trait](../../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| Trait 对象 | [17.2 trait 对象](../../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| 单态化 vs 动态分发 | [Item 12](../../Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md)（ER） |

---

## 一句话

**回调 API 优先 `Fn*`，少用裸 `fn`**

---

## 专项笔记（按需点开）

| # | 专题 | 阅读 |
|---|------|------|
| 01 | 核心知识点 | [01-core-concepts.md](./01-core-concepts.md) |
| 02 | 逻辑脉络 | [02-logic-flow.md](./02-logic-flow.md) |
| 03 | 重点结论 | [03-key-takeaways.md](./03-key-takeaways.md) |
| 04 | 案例与代码 | [04-examples.md](./04-examples.md) |
| 05 | 易错细节 | [05-pitfalls.md](./05-pitfalls.md) |
| 06 | `FnOnce<()>` 与 `'env` 辨析 | [06-trait-generic-params.md](./06-trait-generic-params.md) |
| 07 | 尖括号：生命周期 vs 类型参数 | [07-lifetime-vs-type-in-angle-brackets.md](./07-lifetime-vs-type-in-angle-brackets.md) |
| 08 | `'env` 与 `Scope`（不是 trait） | [08-scope-env-lifetime.md](./08-scope-env-lifetime.md) |
| 09 | 方法与三种 `self` | [09-methods-and-self.md](./09-methods-and-self.md) |
| — | 背诵提纲 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

```text
自由函数 → 方法（绑在类型上）
    → fn（无环境）
    → 闭包（带环境 + Fn*）
    → Trait（多态抽象）
         ├─ 泛型 + Trait Bound → 单态化 / 静态分发
         └─ dyn Trait → vtable / 动态分发
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 02](../../ER-拓展索引.md#item-02)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
