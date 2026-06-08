# Item 1: Use the type system to express your data structures

> **Effective Rust** · [Chapter 1 — Types](../../ER-本书目录.md)  
> **中文**：使用类型系统表达数据结构  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 标量、数组、元组 | [3.2 数据类型](../../Book/03-common-concepts/3.2-数据类型.md) |
| 结构体 | [5.1 结构体](../../Book/05-structs/5.1-定义并实例化结构体.md) |
| 枚举、ADT、`Option` | [6.1 定义枚举](../../Book/06-enums-pattern-matching/6.1-定义枚举.md) |
| `Result`、错误处理 | [9.2 Result](../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |

---

## 一句话

**用类型表达设计意图，让无效状态在编译期写不出来**（Make invalid states inexpressible）。

---

## 专项笔记（按需点开）

| 专题 | 内容 | 阅读 |
|------|------|------|
| 01 | 整数、无隐式转换 | [topics/01-fundamental-types.md](./topics/01-fundamental-types.md) |
| 02 | `bool` / 浮点 / `()` / `char` | [topics/02-scalar-types.md](./topics/02-scalar-types.md) |
| 03 | 数组、元组、结构体 | [topics/03-aggregate-types.md](./topics/03-aggregate-types.md) |
| 04 | 枚举与 ADT | [topics/04-enum-adt.md](./topics/04-enum-adt.md) |
| 05 | `Option` / `Result` | [topics/05-option-result.md](./topics/05-option-result.md) |
| 06 | 设计模式与重构案例 | [topics/06-design-patterns.md](./topics/06-design-patterns.md) |
| 07 | 易错细节 | [topics/07-pitfalls.md](./topics/07-pitfalls.md) |
| — | 背诵提纲（考前速览） | [topics/cheat-sheet.md](./topics/cheat-sheet.md) |

---

## 逻辑脉络

```text
严格基础类型 → 静态布局（struct / tuple）
      → 带数据的 enum 表达业务状态
      → match 穷尽性检查
      → Option / Result + ? 统一错误传播
```

- **类型安全递进**：编译期捕获无效状态，而非运行时兜底。
- **穷尽性 `match`**：必须处理每个变体，否则不编译。
- **与 Item 3、4 衔接**：转换方法 + `?` + 错误类型设计。

---

## 后续拓展

> [ER-拓展索引 § Item 01](../../ER-拓展索引.md#item-01)
