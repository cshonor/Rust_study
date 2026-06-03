# Item 14: Understand lifetimes

> **Effective Rust** · [Chapter 3 — Concepts](../ER-本书目录.md)  
> **中文**：理解生命周期  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| **10.3 专题（主入口）** | [10.3 索引](../../Book/10-generics-traits-lifetimes/10.3-生命周期与引用有效性.md) |
| 悬垂引用 | [10.3.1](../../Book/10-generics-traits-lifetimes/10.3.1-悬垂引用.md) |
| 同 `'a`、红线 | [10.3.2](../../Book/10-generics-traits-lifetimes/10.3.2-同a约束与红线.md) |
| 基础、省略 | [10.3.3](../../Book/10-generics-traits-lifetimes/10.3.3-生命周期基础.md)、[10.3.5 显式/隐式](../../Book/10-generics-traits-lifetimes/10.3.5-显式与隐式生命周期.md) |
| `longest` 等 | [10.3.4](../../Book/10-generics-traits-lifetimes/10.3.4-longest与get_first.md) |
| 结构体、`'static` | [10.3.6](../../Book/10-generics-traits-lifetimes/10.3.6-结构体-static与泛型.md) |
| 借用检查器 | [Item 15](./Item-15-borrow-checker.md)（ER） |
| `const` / `'static` | [4.1 const 与 static](../../Book/04-ownership/4.1-const与static.md) |

---

## 1. 核心知识点与关键定义

### 生命周期是什么

- 保证**引用在解引用时仍有效**，杜绝悬垂指针。
- 每个引用都有生命周期（如 `'a`）；多数场景可**省略**（elision），但机制仍在。

### 栈与作用域

- 栈上变量：从绑定到具名变量开始 → **move 走**或出 `{}` **drop** 结束。
- **NLL（非词法生命周期）**：借用检查器看**最后一次使用**，可早于 `}` 结束借用，不必死等到块尾。

### `'static`

- **唯一有固定名字**的生命周期（其余多为占位 `'a`）。
- 数据**永不被销毁**：字符串字面量、静态变量、`Box::leak` 等（见案例）。

### 匿名生命周期 `'_`

- 「这里依赖某输入的生命周期，但名字不重要」→ `ReferenceHolder<'_>`。

---

## 2. 逻辑脉络

```text
C/C++ 隐式指针寿命 → UAF
Rust：生命周期进类型系统 → 编译期拒绝悬垂

堆数据存活可变，但 owner 链终落在：
  栈局部变量 或 'static
→ 堆上引用仍受栈/owner 寿命约束

输出生命周期 'a：通常 = 输入生命周期的交集（最短者包含输出）
```

> 细则与 `longest` 见 Book **10.3.2**、**10.3.4**。

---

## 3. 重点结论与实用要点

### 生命周期省略（Elision）三条

| # | 情形 | 编译器假定 |
|---|------|------------|
| 1 | 一个输入 + 有输出 | 输出寿命 = 该输入 |
| 2 | 多个输入、无输出 | 各输入**独立**寿命 |
| 3 | 有 `&self` / `&mut self` + 有输出 | 输出寿命 = **`self`** |

（完整版与例外见 Book **10.3.5**。）

### 生命周期「传染」

- struct 里带引用 → 必须 `Struct<'a>` → 外层再嵌套也要带 `'a`。
- **优先 owned 数据**（`String`、`Vec`）；要共享用 **`Rc`/`Arc`** 解耦刚性借用（Item 15、20）。

---

## 4. 案例与代码要点

### `Box::leak` → `'static`

```rust
let boxed = Box::new(Item { contents: 12 });
let r: &'static Item = Box::leak(boxed);
// 堆不再被 Drop；引用可视为 'static（慎用：泄漏）
```

### 返回带引用的 struct：用 `'_`

```rust
// 不推荐：隐藏与 items 的绑定
// pub fn find_one(items: &[Item]) -> ReferenceHolder

pub fn find_one(items: &[Item]) -> ReferenceHolder<'_> {
    // ...
}
```

### 临时值 + 借用

```rust
// fn_returning_ref(&mut Item { contents: 42 }) // 临时在语句末 drop
let mut tmp = Item { contents: 42 };
let r = fn_returning_ref(&mut tmp); // ✅ 先绑定具名变量
```

---

## 5. 易错细节

| 问题 | 说明 |
|------|------|
| **`const` 即 `'static`？** | 并非所有常量引用都是 `'static`；含 **Drop** 或 **内部可变性** 等可能不行 |
| **临时值 dropped while borrowed** | 匿名临时只活到**表达式结束**；需 `let` 延长 |
| **同 `'a` 误解** | 不是「变量一起死」；是**输出引用**不能活过输入允许的范围（见 10.3.2） |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 14](../ER-拓展索引.md#item-14)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 本质 | 引用有效范围，编译期检查 |
| NLL | 借用到**最后使用**即可结束 |
| `'static` | 永不过期；字面量、leak |
| `'_` | 匿名，提示「借来的」 |
| 省略 | 单输入→输出同寿；有 self→跟 self |
| 设计 | 优先 owned；引用会传染 |
