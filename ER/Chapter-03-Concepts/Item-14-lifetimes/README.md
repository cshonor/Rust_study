# Item 14: Understand lifetimes

> **Effective Rust** · [Chapter 3 — Concepts](../../ER-本书目录.md)  
> **中文**：理解生命周期  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| **10.3 专题（主入口）** | [10.3 索引](../../../Book/10-generics-traits-lifetimes/10.3-生命周期与引用有效性.md) |
| 悬垂引用 | [10.3.1](../../../Book/10-generics-traits-lifetimes/10.3.1-悬垂引用.md) |
| 同 `'a`、红线 | [10.3.2](../../../Book/10-generics-traits-lifetimes/10.3.2-同a约束与红线.md) |
| 基础、省略 | [10.3.3](../../../Book/10-generics-traits-lifetimes/10.3.3-生命周期基础.md)、[10.3.5 显式/隐式](../../../Book/10-generics-traits-lifetimes/10.3.5-显式与隐式生命周期.md) |
| `longest` 等 | [10.3.4](../../../Book/10-generics-traits-lifetimes/10.3.4-longest与get_first.md) |
| 结构体、`'static` | [10.3.6](../../../Book/10-generics-traits-lifetimes/10.3.6-结构体-static与泛型.md) |
| 借用检查器 | [Item 15](../Item-15-borrow-checker/README.md)（ER） |
| `const` / `'static` | [4.1 const 与 static](../../../Book/04-ownership/4.1-const与static.md) |

---

## 一句话

**优先 owned 数据**

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
C/C++ 隐式指针寿命 → UAF
Rust：生命周期进类型系统 → 编译期拒绝悬垂

堆数据存活可变，但 owner 链终落在：
  栈局部变量 或 'static
→ 堆上引用仍受栈/owner 寿命约束

输出生命周期 'a：通常 = 输入生命周期的交集（最短者包含输出）
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 14](../../ER-拓展索引.md#item-14)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
