# Item 1: Use the type system to express your data structures

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：使用类型系统表达数据结构  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 标量、数组、元组 | [3.2 数据类型](../../Book/03-common-concepts/3.2-数据类型.md) |
| 结构体 | [5.1 结构体](../../Book/05-structs/5.1-定义并实例化结构体.md) |
| 枚举、ADT、`Option` | [6.1 定义枚举](../../Book/06-enums-pattern-matching/6.1-定义枚举.md) |
| `Result`、错误处理 | [9.2 Result](../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |

---

## 1. 核心知识点与关键定义

### 基础类型严谨性（Fundamental Types）

- Rust 内建整数有明确大小（`i32`、`u8`）及平台相关类型（`isize`、`usize`）。
- **编译器极其严格**：不允许隐式类型转换；较小整数赋给较大类型也会报错，必须显式处理截断风险。

### 字符与特殊类型

| 类型 | 要点 |
|------|------|
| `bool` | 布尔 |
| `f32` / `f64` | 浮点 |
| `()` | 单元类型，类似 C 的 `void` |
| `char` | **Unicode 标量**（4 字节），不能与 `i32` 等静默互转 |

### 聚合类型（Aggregate Types）

| 类型 | 说明 |
|------|------|
| **数组** `[T; N]` | 编译期定长、同构 |
| **元组** | 定长、异构 |
| **结构体** `struct` | 命名字段 |
| **元组结构体** | 有类型名，字段用 `.0`、`.1` 访问 |

### 枚举与代数数据类型（ADT）

- 枚举变体可**携带不同类型、不同数量的数据** → **代数数据类型（ADT）**。
- 可理解为：C 枚举 + **类型安全 union** 的结合。

### 内建枚举

| 类型 | 语义 |
|------|------|
| **`Option<T>`** | 值可能存在 `Some(T)` 或缺失 `None` |
| **`Result<T, E>`** | 成功 `Ok(T)` 或失败 `Err(E)` |

---

## 2. 逻辑脉络与知识点关联

```text
严格基础类型 → 静态布局（struct / tuple）
      → 带数据的 enum 表达业务状态
      → match 穷尽性检查
      → Option / Result + ? 统一错误传播
```

- **类型安全递进**：让编译器在**编译期**捕获无效状态，而非运行时兜底。
- **穷尽性 `match`**：必须处理每个 `enum` 变体，否则不编译。
- **与错误处理协同**：`Option` / `Result` + 转换方法 + `?`，减少样板代码（Item 3、4 展开）。

---

## 3. 重点结论与实用要点

1. **用类型表达设计意图**——对人、对编译器都清晰。
2. **让无效状态无法被类型表达（Make invalid states inexpressible）**——合法组合才写得出来。
3. **缺失值用 `Option<T>`**——不用 `-1`、`nullptr` 等哨兵。
4. **可失败操作用 `Result<T, E>`**——不用魔法返回值或全局错误码。

---

## 4. 案例与代码要点

### 隐式转换会失败

```rust
let x: i32 = 42;
let y: i16 = x; // ❌ mismatched types
```

### 反模式 → 用 enum 消灭非法组合

**重构前**（靠注释约定，字段可处于非法组合）：

```rust
struct ScreenColor {
    monochrome: bool,
    fg_color: RgbColor, // 注释：monochrome 为 true 时 fg 应为 0
}
```

**重构后**（无效组合在类型层面不存在）：

```rust
enum Color {
    Monochrome,
    Foreground(RgbColor),
}
struct ScreenColor {
    color: Color,
}
```

### 布尔参数 → 具名枚举

```rust
// ❌ 易填错、顺序可颠倒而不报错
// print_page(true, false);

// ✅ 意图清晰，错序/错值更易被类型拦住
enum Sides { One, Both }
enum Output { BlackAndWhite, Color }
fn print_page(sides: Sides, output: Output) { /* ... */ }
```

---

## 5. 易错细节

### `Option` 与「空集合」

| 表达 | 何时用 |
|------|--------|
| `Vec<T>` / `String` 为空 | 通常已表示「没有内容」 |
| `Option<Vec<T>>` | 必须区分 **未提供** vs **提供了空集合** 时才嵌套 |

### 公开 `enum` 加变体 = 破坏性变更

- 外部 `match` 因**穷尽性**未覆盖新变体 → **编译失败**。
- 仅需 C 风格常量扩展时，可考虑 `#[non_exhaustive]`（调用方须留 `_` 等兜底分支）。

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 01](../ER-拓展索引.md#item-01)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 核心主张 | 设计写进类型，无效状态编译不过 |
| ADT | `enum` 变体可带数据 |
| 哨兵 | 用 `Option`，别用魔法值 |
| 失败 | 用 `Result`，别用特殊返回码 |
| `match` | 穷尽；公开 enum 加变体要小心破坏性 |
