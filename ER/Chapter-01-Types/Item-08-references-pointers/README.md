# Item 8: Familiarize yourself with reference and pointer types

> **Effective Rust** · [Chapter 1 — Types](../../ER-本书目录.md)  
> **中文**：熟悉引用与指针类型  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `&T` / `&mut T`、借用 | [4.2 引用与借用](../../../Book/04-ownership/4.2-引用与借用.md) |
| 切片 `&[T]` | [4.3 切片](../../../Book/04-ownership/4.3-切片slice.md) |
| `Box` | [15.1 Box](../../../Book/15-smart-pointers/15.1-使用Box指向堆上的数据.md) |
| `Deref` | [15.2 Deref](../../../Book/15-smart-pointers/15.2-通过Deref将智能指针当作引用.md) · [15.2.1 嵌套/坑点](../../../Book/15-smart-pointers/15.2.1-Deref嵌套可变与编译坑.md) |
| `Rc` / `RefCell` / `Weak` | [15.4 Rc](../../../Book/15-smart-pointers/15.4-Rc引用计数智能指针.md)、[15.5 RefCell](../../../Book/15-smart-pointers/15.5-RefCell与内部可变性.md)、[15.6 循环引用](../../../Book/15-smart-pointers/15.6-引用循环与Weak.md) |
| `Arc` / `Mutex` | [16.3 共享状态](../../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| `dyn Trait` | [17.2 trait 对象](../../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| `AsRef` / 转换 | [Item 5](../Item-05-type-conversions/README.md) |

---

## 一句话

**安全 Rust 无悬垂引用**

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

## 后续拓展

> 展开版：[ER-拓展索引 § Item 08](../../ER-拓展索引.md#item-08)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
