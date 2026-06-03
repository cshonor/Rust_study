# Item 24: Re-export dependencies whose types appear in your API

> **Effective Rust** · [Chapter 4 — Dependencies](../ER-本书目录.md)  
> **中文**：重新导出 API 中出现其类型的依赖项  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `pub use` | [7.4 use 与 pub use](../../Book/07-packages-modules/7.4-使用use关键字将名称引入作用域.md) |
| 对外 API 门面 | [14.2 发布 crate](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| 工作空间依赖解析 | [14.3 Cargo 工作空间](../../Book/14-cargo-crates/14.3-Cargo工作空间.md) |
| Semver 与公开 API | [Item 21](./Item-21-semver.md) |
| 最小可见性 | [Item 22](./Item-22-minimize-visibility.md) |
| 依赖图 / duplicates | [Item 25](./Item-25-dependency-graph.md)（待补） |
| CI 监控 API 边界 | [Item 32](../Chapter-05-Tooling/Item-32-ci.md)（待补） |

---

## 1. 核心知识点与关键定义

### 公开依赖（Public dependency）

- 库 crate 的**公开 API**（函数签名、返回类型、公共 struct 字段等）直接出现**外部 crate 的类型** → 该依赖成为**公开依赖**。

### Cargo 多版本共存

- 同一二进制依赖图中，可**同时链接**同一 crate 的多个**不兼容**版本（如 `rand 0.7` + `rand 0.8`）。
- 同名不同类型 → Rust 视为**两种互不兼容的类型**。

### 重导出（Re-export）

```rust
pub use some_dependency;
// 或
pub use some_dependency::SomeType;
```

- 把依赖（或其类型）作为**本 crate 公开 API 的一部分**暴露给下游。

---

## 2. 逻辑脉络

```text
dep-lib API 用 rand 0.7 的类型
app 直接用 rand 0.8 构造对象
         ↓
同名 ThreadRng，不同版本 → 类型断层，无法传入 API
         ↓
用户被迫写 wrapper crate 对齐版本（极烦）
         ↓
库作者：pub use rand; → 下游用 dep_lib::rand 拿「对的那版」
```

与 Item 21：公开依赖的类型若依赖升 **Major**，你的 crate 通常也须 **Major**。

---

## 3. 重点结论与实用要点

### 极度谨慎：在 API 里暴露外部类型

- 决定前**三思**：绑定该依赖的 Semver 节奏。
- 仅对**极稳定、生态基石、依赖少**的库（如 `rand`）较可接受。
- 更优：用**自有 newtype** 包装，隐藏具体依赖（未在本文展开）。

### 若 API 已暴露依赖类型 → **必须重导出**

- `pub use` 相关类型或整个 crate。
- 让下游能**构造、匹配**与你 API 一致的类型，而非猜错版本。

---

## 4. 案例与代码要点

### 跨版本调用失败

```toml
# app
rand = "=0.8.5"
dep-lib = "0.1"  # 内部 rand = "=0.7.3"
```

```rust
// dep-lib
pub fn pick_number_with<R: rand::Rng>(rng: &mut R, n: usize) -> usize { /* ... */ }

// app
let mut rng = rand::thread_rng(); // 0.8
let max: usize = rng.gen_range(5..10);
let choice = dep_lib::pick_number_with(&mut rng, max); // ❌ 类型不匹配
```

### 重导出破局

```rust
// dep-lib/lib.rs
pub use rand;

pub fn pick_number_with<R: rand::Rng>(rng: &mut R, n: usize) -> usize { /* ... */ }
```

```rust
// app
let mut prev_rng = dep_lib::rand::thread_rng(); // 0.7，与库一致
let choice = dep_lib::pick_number_with(&mut prev_rng, max); // ✅
```

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **误导性 trait 报错** | `ThreadRng: RngCore is not satisfied` — 常是 **0.8 的 RngCore vs 0.7 的 RngCore**，不是「忘 import trait」 |
| **未重导出** | 下游只能用自己 Cargo.toml 里的版本构造 →  silent 版本错配 |
| **公开依赖泄漏** | PR 里无意把私有依赖类型放进 pub fn → 用 `cargo-public-api` 等 CI 监控（§6） |

排查：**先 `cargo tree --duplicates`**（Item 25）看同名 crate 多版本。

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 24](../ER-拓展索引.md#item-24)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 公开依赖 | API 里出现外部类型 = 与其 Semver 绑定 |
| 多版本 | 同名不同版 = **不同类型** |
| 必须做 | API 用了就要 **`pub use`** |
| 下游 | 用 `your_crate::dep` 构造匹配类型 |
| 报错 | trait not satisfied → 先查**版本重复** |
| 谨慎 | 能不用外部类型在 API 里就别用 |
