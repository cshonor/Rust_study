# Item 28: Use macros judiciously

> **Effective Rust** · [Chapter 5 — Tooling](../ER-本书目录.md)  
> **中文**：明智地使用宏  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 声明宏 / 过程宏、`macro_rules!` | [19.5 宏](../../Book/19-advanced-features/19.5-宏.md) |
| 泛型 vs 宏的抽象层级 | [10.1 泛型](../../Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md)、[Item 12](../Chapter-02-Traits/Item-12-generics-vs-trait-objects.md) |
| derive 代替运行时反射 | [Item 19](../Chapter-03-Concepts/Item-19-avoid-reflection.md) |
| 过程宏供应链风险 | [Item 25](../Chapter-04-Dependencies/Item-25-dependency-graph.md) |
| `cargo-expand` 等 | [Item 31](./Item-31-tooling-ecosystem.md)（待补） |

---

## 1. 核心知识点与关键定义

### 元编程（Metaprogramming）

- **写生成代码的代码**；宏用于消除**确定、重复**的样板。

### 宏的类型

| 类别 | 形式 | 说明 |
|------|------|------|
| **声明式** | `macro_rules!` | macros by example；按 token 角色匹配插入 |
| **过程宏** | 独立 `proc-macro` crate | 操作 **TokenStream** |

过程宏三种：

1. **类函数** — `foo!(...)`
2. **属性** — `#[foo]`
3. **派生** — `#[derive(Foo)]`（**最常用**）

### 卫生性（Hygienic）

- 声明式宏展开**不会**意外捕获调用点局部变量 → 避免 C 宏式命名冲突。

---

## 2. 逻辑脉络

```text
函数：     同类型不同值
泛型+trait：不同类型
宏：         同「语法角色」的不同程序片段（ident / expr / ty…）
         ↓
Rust 宏：Token / AST 级（非 C 文本替换）
         ↓
代价：难读、难调试、rustfmt/IDE 黑盒、编译期执行（Item 25）
         ↓
原则：函数/泛型不够再用；优先 derive
```

---

## 3. 重点结论与实用要点

### 只在无可替代时用宏

- 先：**函数** → **泛型 + trait** → 最后 **宏**。

### 优先派生宏

- 为 struct/enum **每个字段/变体**生成代码 → **`#[derive(...)]`**，而非整类型 `foo!()` 生成。

### 对齐普通 Rust 直觉

| 避免 | 推荐 |
|------|------|
| 宏内隐式 `return` / `?` | 控制流像普通代码一样可见 |
| `my_macro!(list)` 内部偷偷加 `&` | `my_macro!(&list)` 调用方显式引用 |
| 自拼格式化 | 用 **`format_args!`**（如 `println!` 同款） |

---

## 4. 案例与代码要点

### 微型 DSL：HTTP 状态码

- 一次宏调用集中：**数值 + 分组 + 文案**。
- 展开 → enum、`TryFrom`、分组函数、文本 lookup — **单一事实来源**。

### 注入诊断位置

```rust
// 展开处自动带 file!() / line!()
panic!("assert failed at {}:{}", file!(), line!());
```

宏在**调用点**展开 → 位置信息准确，无需手传。

### 重复求值陷阱（声明宏）

```rust
macro_rules! square {
    ($e:expr) => { { $e * $e } };
}
// square!( { x += 1; x } )  → x += 1 执行两次！
```

修复：

```rust
($e:expr) => { { let __val = $e; __val * __val } };
// 或限制为 $i:ident
```

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **`$e:expr` 用多次** | 副作用 / 昂贵计算重复 |
| **可见性** | 宏只对定义**之后**可见；`#[macro_export]` → **crate 根** |
| **工具链** | 展开代码难 `rustfmt`；IDE 跳转差；一行 → 数百行 **code bloat** |
| **proc-macro crate** | 编译时任意 Rust 代码 → 依赖审计（Item 25） |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 28](../ER-拓展索引.md#item-28)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 顺序 | 函数 → 泛型 → **宏** |
| 过程宏 | 优先 **`#[derive]`** |
| 声明宏 | **卫生**；注意 **expr 重复求值** |
| 控制流 | 别藏 `return`/`?` |
| 调试 | `cargo-expand` |
| 代价 | IDE/rustfmt/体积/供应链 |
