# Item 15: Understand the borrow checker

> **Effective Rust** · [Chapter 3 — Concepts](../ER-本书目录.md)  
> **中文**：理解借用检查器  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-15-borrow-checker](../ER-demos/item-15-borrow-checker/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 借用规则 | [4.2 引用与借用](../../Book/04-ownership/4.2-引用与借用.md) |
| 生命周期、NLL | [Item 14](./Item-14-lifetimes.md)、[10.3 专题](../../Book/10-generics-traits-lifetimes/10.3-生命周期与引用有效性.md) |
| `RefCell` / 内部可变 | [15.5 RefCell](../../Book/15-smart-pointers/15.5-RefCell与内部可变性.md) |
| `Arc` / `Mutex` | [16.3 共享状态](../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| 智能指针选型 | [Item 8](../Chapter-01-Types/Item-08-references-pointers.md) |

---

## 1. 核心知识点与关键定义

### 借用（Borrowing）

- 值有**唯一所有者**；通过 **引用** 把访问权「借出」给其它代码。

### 访问模型（CRUD 视角）

| 角色 | 权限 |
|------|------|
| **Owner** | Create / Read / Update / **Move·Drop**（**只有 owner 能 move**） |
| **`&mut T`** | Read + Update |
| **`&T`** | Read only |

### 借用双原则

1. **生命周期**：引用存活期 ⊆ 所指数据存活期（**NLL**：常到**最后使用**为止，见 Item 14）。
2. **读写互斥**：同一时刻对同一数据 —— **多个 `&T`** **或** **唯一一个 `&mut T`**，不能兼有。

### 别名与优化（Aliasing）

- 活跃 **`&T`** 期间，编译器假定内存**不会被其它 `&mut` 偷偷改** → 可激进优化（寄存器缓存等）。
- 同时从编译期排除多线程 **data race**（再配合 `Send`/`Sync`，Item 17）。

---

## 2. 逻辑脉络

```text
C/C++ 裸指针：无借期、无读写互斥 → UAF / 数据竞争
Rust 引用 & / &mut + 借用检查器 → 编译期拦截

出借期间 owner 被压制：
  存在 &T   → owner 可读，不可改 / move / drop
  存在 &mut → owner 连读都不行，直到 &mut 结束
```

---

## 3. 重点结论与实用要点

### 和借用检查器「和解」的三招

| 策略 | 做法 |
|------|------|
| **延长** | `let tmp = ...` 绑定临时值，避免 temporary dropped while borrowed |
| **缩短** | 内层 `{ ... }` 让引用**提前**结束 |
| **拆链** | 复杂 `Option`/`Rc`/`RefCell`/`map` 链 → 逐步 `let` + 类型标注，定位冲突 |

### 数据结构设计

| 问题 | 方向 |
|------|------|
| **自引用**（字段 A 拥有 `String`，字段 B 存 `&str` 指 A） | ❌ move 即炸；用 **range / 索引**、专用 crate，或 **Pin**（§6） |
| **图/树多引用** | 放弃裸引用 → **`Rc<RefCell<T>>`**（单线程）或 **`Arc<Mutex<T>>`**（多线程） |

---

## 4. 案例与代码要点

### `mem::replace` / `Option::replace`

```rust
// item: &mut Option<Item>
// let prev = *item; // ❌ cannot move out of *item

let prev = std::mem::replace(item, Some(new_val));
// 或
let prev = item.replace(new_val);
```

在**仍持有 `&mut`** 的前提下，原子地「取出旧值、写入新值」。

### 临时 `String` 逃逸

```rust
// let found = find(&format!("..."), "ex"); // ❌ format! 临时在 ; 处 drop

let query = format!("...");
let found = find(&query, "ex");
```

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **format! / 临时 + 借引用** | 表达式结束临时 drop → 悬垂借用 |
| **多索引结构不同步** | `Vec` + `BTreeMap` 互存引用或脆弱 index，删除时易乱 → 智能指针或稳定 ID |
| **owner 仍想用数据** | 有 outstanding 借用时不能 move/drop；有 `&mut` 时 owner 不能读 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 15](../ER-拓展索引.md#item-15)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 规则 1 | 引用不能活过数据 |
| 规则 2 | 多个 `&` **或** 一个 `&mut` |
| Owner | 借出期间权限被压；只有 owner 能 move |
| 和解 | 延长 / 缩短 / 拆链 |
| 图/自引用 | `Rc<RefCell<_>>` 或索引；别硬裸引用 |
| replace | 从 `&mut Option` 里安全换值 |
