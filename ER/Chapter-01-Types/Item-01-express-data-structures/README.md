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

## 1. 核心知识点与关键定义

> **总原则（背一句）**：强静态类型 · 编译期检查 · **无隐式数值转换** · **无 null**（用 `Option`）。

### 1.1 基础类型严谨性（Fundamental Types）

**定义**：Rust 是**强静态类型**语言，所有基础类型有**明确位数与符号**，类型安全由编译器强制保证。

**整数类型**

| 分类 | 类型 |
|------|------|
| 固定大小 | `i8` / `i16` / `i32` / `i64` / `i128`，`u8` / `u16` / `u32` / `u64` / `u128` |
| 平台相关 | `isize` / `usize`（等于指针宽度，32 或 64 位） |

**关键规则**

- ❌ **无隐式类型转换**（`u8 → u32` 也必须 `as` 或 `From`/`Into`）
- ❌ 不同类型不能直接比较、赋值
- ✅ 必须显式处理截断、溢出风险（`as` 截断；溢出 debug 下 panic，可用 `wrapping_*` / `checked_*` 等）

```rust
let a: u8 = 10;
let b: u32 = a as u32; // 必须显式
// let c: u32 = a;     // ❌ mismatched types
```

> **面试补一句**：「无隐式转换」指数值/基础类型层面；另有 Deref 强制转换、trait 对象等，别和整数混谈。

### 1.2 字符与标量类型（Scalar）

标量 = **单个值**的类型。

| 类型 | 要点 |
|------|------|
| `bool` | `true` / `false`，1 字节 |
| `f32` / `f64` | IEEE 754 浮点，32 / 64 位 |
| `()` | **单元类型**，唯一值 `()`；表示「无有意义返回值」，语义类似 C 的 `void` |
| `char` | **Unicode 标量值**，固定 **4 字节**；**不能与整数静默互转** |

```rust
let c: char = '中'; // 4 字节
// let x: i32 = c;  // ❌ 不允许
```

### 1.3 聚合类型（Aggregate Types）

**定义**：把多个值组合成一个整体的类型。

| 类型 | 背一句 |
|------|--------|
| **数组** `[T; N]` | **编译期定长**、**同构**；`arr[i]` 越界 → **panic**（安全设计，非 C 式静默读错） |
| **元组** `(T1, T2, …)` | 定长、**异构**；`.0`、`.1` 访问；可返回多个值 |
| **结构体** `struct` | **命名字段**，自定义复合数据 |
| **元组结构体** | 有类型名、无字段名；`.0`、`.1` 访问 |

```rust
struct Point { x: i32, y: i32 }
struct Color(u8, u8, u8); // 元组结构体
```

### 1.4 枚举与代数数据类型（ADT）

**定义**：**枚举 = ADT**——一个类型只能是**若干互斥变体之一**，变体可携带**不同类型、不同数量**的数据。

**本质**：C 的 tag enum + **类型安全** tagged union。

| 变体形式 | 示例 |
|----------|------|
| 单元变体 | `Quit` |
| 元组变体 | `Write(String)` |
| 结构体变体 | `Move { x: i32, y: i32 }` |

```rust
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}
```

### 1.5 内建枚举

| 类型 | 变体 | 语义 | 替代什么 |
|------|------|------|----------|
| **`Option<T>`** | `Some(T)` / `None` | 值可能存在或缺失 | **null**、空指针 |
| **`Result<T, E>`** | `Ok(T)` / `Err(E)` | 成功或失败 | **异常**（可恢复错误） |

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

> 展开版：[ER-拓展索引 § Item 01](../../ER-拓展索引.md#item-01)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片 · 极简背诵提纲

### 四句话总览（默写版）

1. **基础类型**：位数明确、强类型、整数 / `char` **不隐式转换**。
2. **聚合**：数组同构定长，元组异构定长，结构体命名字段。
3. **enum**：互斥变体 + 可带数据 = **ADT**。
4. **Option / Result**：无 null；错误必须显式处理。

### 速查表

| 块 | 背一句 |
|----|--------|
| 整数 | `i8~i128` / `u8~u128` 定长；`isize`/`usize` = 指针宽 |
| 标量 | `bool` · `f32/f64` · `()` · `char`（4B Unicode 标量） |
| 数组 | `[T; N]` 编译期定长同构；越界 panic |
| 元组 | 定长异构；`.0` `.1` |
| struct | 命名字段；元组结构体用 `.0` |
| ADT | 单元 / 元组 / 结构体变体，互斥 |
| `Option` | `Some` / `None` → 无 null |
| `Result` | `Ok` / `Err` → 可恢复错误 |

| 要点 | 一句 |
|------|------|
| 核心主张 | 设计写进类型，无效状态编译不过 |
| ADT | `enum` 变体可带数据 |
| 哨兵 | 用 `Option`，别用魔法值 |
| 失败 | 用 `Result`，别用特殊返回码 |
| `match` | 穷尽；公开 enum 加变体要小心破坏性 |
