# Item 27: Document public interfaces

> **Effective Rust** · [Chapter 5 — Tooling](../ER-本书目录.md)  
> **中文**：为公共接口编写文档  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `///` / `//!`、doc test | [14.2 发布到 Crates.io](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| 类型表达语义 | [Item 1](../Chapter-01-Types/Item-01-express-data-structures/README.md) |
| `# Panics` / 少 panic | [Item 18](../Chapter-03-Concepts/Item-18-dont-panic/README.md) |
| CI 集成 doc | [Item 32](../Item-32-ci/README.md)（待补） |
| 工具生态 | [Item 31](../Item-31-tooling-ecosystem/README.md)（待补） |
| 最小可见性 | [Item 22](../Chapter-04-Dependencies/Item-22-minimize-visibility/README.md) |

---

## 1. 核心知识点与关键定义

### 文档注释

| 语法 | 作用 |
|------|------|
| **`///`** | 说明**紧下方**的项（fn、struct…） |
| **`//!`** | 说明**包含**该注释的模块 / crate（如 `lib.rs` 顶） |

- 内容：**Markdown**。

### 约定板块（Sections）

| 板块 | 用途 |
|------|------|
| **`# Examples`** | 用法示例（可 doc test） |
| **`# Panics`** | 触发 `panic!` 的条件 |
| **`# Errors`** | `Result` 可能的错误 |
| **`# Safety`** | `unsafe` 前置条件 |

### Doc tests

- `# Examples` 里 **fenced code block** → `cargo test` 自动编译运行 → 文档与代码同步。

### 生成与托管

| 命令 / 站点 | 作用 |
|-------------|------|
| `cargo doc --open` | 本地 HTML API 文档 |
| **docs.rs** | 发布后自动构建托管 API 文档 |
| **crates.io** | 展示 crate 顶层 **README.md** |

---

## 2. 逻辑脉络

```text
Item 1：能进类型的语义 → 进类型（编译器强制）
         ↓
文档：只补类型表达不了的「为什么 / 契约 / 陷阱」
         ↓
doc test + CI cargo doc → 防示例与链接退化（Item 32）
         ↓
examples/ → 完整集成示例，非 doc test 片段
```

---

## 3. 重点结论与实用要点

### 不要复述签名

- ❌ 「接收 `&BoundingBox`，返回新 `BoundingBox`」—— 签名已说清，重构易过时。
- ✅ **意图、不变量、边界、为何这样设计**。

### 写「为什么」，不写「谁在用」

- ❌ 「模块 A 目前用此方法做 X」—— A 重构后文档即垃圾。
- ✅ **面向未来的语义**与使用约束。

### Markdown 与交叉引用

- 标识符用 `` `code` ``；链接用 `` [`SomeType`] `` → rustdoc 可跳转。

### 编译器护栏

```rust
#![deny(broken_intra_doc_links)]  // 死链 → 编译错误
#![warn(missing_docs)]            // pub 项缺文档 → 警告
```

---

## 4. 案例与代码要点

### 死链检测

```rust
#![deny(broken_intra_doc_links)]

/// The bounding box for a [`Polygone`].  // 拼写错误
// → unresolved link to `Polygone`
```

### `examples/` 目录

- 完整可运行集成示例，**只调 public API**。
- `cargo run --example demo_name`
- 区别于 `tests/`（可测私有）和 doc test 短片段。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **为关 warning 写废话** | `missing_docs` 逼出的垃圾注释 **比没有更糟** |
| **漏 `# Panics` / `# Safety`** | 违反 Item 18 或含 `unsafe` 却不写 → 最小惊吓原则破产 |
| **只写 crate 级 README** | crates.io 看 README；API 细节在 **docs.rs** + `//!` 模块文档 |
| **examples 测不到** | 长示例放 `examples/`，CI 应 `cargo test --examples` 或单独跑 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 27](../ER-拓展索引.md#item-27)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 分工 | **类型**表达能表达的；**文档**补 WHY |
| 禁止 | **复述签名** |
| 板块 | Examples / Panics / Errors / Safety |
| 质量 | doc test + `deny(broken_intra_doc_links)` |
| 长示例 | **`examples/`** + `cargo run --example` |
| 发布 | README → crates.io；API → docs.rs |
