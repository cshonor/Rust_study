# Item 7: Use builders for complex types

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：为复杂类型使用建造者模式  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 结构体、字段初始化 | [5.1 结构体](../../Book/05-structs/5.1-定义并实例化结构体.md) |
| `Default`、`..` 更新语法 | [3.1 变量和可变性](../../Book/03-common-concepts/3.1-变量和可变性.md) §Default |
| 方法、`self` / `&mut self` | [5.3 方法语法](../../Book/05-structs/5.3-方法语法.md)、[Item 2](./Item-02-express-common-behavior.md) |
| 所有权、move | [4.1 所有权](../../Book/04-ownership/4.1-什么是所有权.md) |

---

## 1. 核心知识点与关键定义

### 结构体初始化的刚性

- Rust 要求创建 struct 时**每个字段都有值** → 无未初始化内存。
- 多可选字段时，手写 `None` / 默认值 → **样板代码**多，字段增删易漏改。

### `Default` + 结构体更新语法的局限

```rust
let d = Details {
    name: "Bob".into(),
    ..Default::default()
};
```

- **前提**：所有字段类型都实现 `Default`。
- 某字段（如外部库的 `time::Date`）**没有合适默认值** → 此捷径失效。

### Builder 模式

- 单独 **`XxxBuilder`** 存构造中间状态；
- **setter** 逐步填字段；
- **`build()`** 产出最终 `Xxx`。

---

## 2. 逻辑脉络

```text
全字段手写初始化（冗长、易随字段变化腐化）
  → Default + ..（字段都要 Default）
  → Builder（构造逻辑与类型解耦，可选字段友好）
  → derive_builder 等宏（少手写样板）
```

---

## 3. 重点结论：两种 Builder 风格

| | **消费型（Consuming）** | **可变借用型（Mutating）** |
|--|-------------------------|----------------------------|
| setter 签名 | `mut self` → `Self` | `&mut self` → `&mut Self` |
| 链式调用 | ✅ 一气呵成 | ⚠️ 临时值链式会踩生命周期 |
| `if` 分支 | 需 `builder = builder.x()` 重绑 | ✅ `builder.x()` 直接改 |
| `build` | 通常 `build(self)` 消耗 builder | 常 `build(&self)`，可 **Clone 后多次 build** |
| 重复 build | ❌ 一次 `build` 吃掉 builder | ✅ 同一模板可多次产出 |

### 生态

- 手写 Builder 把样板从**调用方**挪到 **Builder 定义**；
- 优先用 **`derive_builder`** 等 crate 自动生成。

---

## 4. 案例与代码要点

### 消费型：顺畅链式 vs `if` 断点

```rust
// 链式
let bob = DetailsBuilder::new("Robert", "Builder", date)
    .middle_name("the")
    .preferred_name("Bob")
    .build();

// if 打断：每次 setter  move self，必须重绑
let mut builder = DetailsBuilder::new("Robert", "Builder", date);
if informal {
    builder = builder.preferred_name("Bob");
}
let bob = builder.build();
```

### 消费型：不能 build 两次

```rust
// vec![smithy.build(), smithy.build()] // ❌ use of moved value
```

### 借用型：临时值早夭

```rust
// ❌ new() 的临时 builder 在本语句结束就 drop，链上 &mut 悬空
// let bob = DetailsBuilder::new(...).middle_name("the").build();

let mut builder = DetailsBuilder::new("Robert", "Builder", date);
builder.middle_name("the");
let bob = builder.build();
```

---

## 5. 易错细节

| 陷阱 | 原因 |
|------|------|
| `use of moved value` | 消费型 `build(self)` 后 builder 已失效 |
| **Temporary dropped while borrowed** | `&mut self` 链在**临时值**上，语句末临时对象 drop |
| 字段校验 | `build()` 里应集中校验（必填、互斥），别散落 setter |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 07](../ER-拓展索引.md#item-07)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 为何 Builder | 多可选字段、无全局 Default |
| 消费型 | 好链式；if 要重绑；build 一次 |
| 借用型 | 好分支；先 `let mut builder` 再改 |
| 宏 | 优先 `derive_builder` |
| 校验 | 放在 `build()` |
