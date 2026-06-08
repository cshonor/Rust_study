# Item 17: Be wary of shared-state parallelism

> **Effective Rust** · [Chapter 3 — Concepts](../../ER-本书目录.md)  
> **中文**：对共享状态的并行保持警惕  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 线程、`spawn` | [16.1 使用线程](../../../Book/16-fearless-concurrency/16.1-使用线程同时运行代码.md) |
| 消息传递（优先） | [16.2 消息传递与通道](../../../Book/16-fearless-concurrency/16.2-消息传递与通道.md) |
| `Mutex` / `Arc<Mutex<T>>` | [16.3 共享状态并发](../../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| `Send` / `Sync` | [16.4 Send 与 Sync](../../../Book/16-fearless-concurrency/16.4-Send与Sync.md) |
| 内部可变性 | [15.5 RefCell](../../../Book/15-smart-pointers/15.5-RefCell与内部可变性.md) |
| 借用与 data race | [Item 15](../Item-15-borrow-checker/README.md) |

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
借用规则（多个 & 或 一个 &mut）≈ 消除 data race 的逻辑前提
         ↓
安全 Rust：编译期无 data race
         ↓
共享可变状态：Arc（多线程引用计数）+ Mutex（内部可变）→ Arc<Mutex<T>>
         ↓
多 Mutex / 锁顺序不一致 → 死锁（编译期不管）
         ↓
对策：优先消息传递；合并 State；极小锁域
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 17](../../ER-拓展索引.md#item-17)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
