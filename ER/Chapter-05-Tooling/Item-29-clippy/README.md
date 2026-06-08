# Item 29: Listen to Clippy

> **Effective Rust** · [Chapter 5 — Tooling](../ER-本书目录.md)  
> **中文**：倾听 Clippy 的建议  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [clippy.toml](../../ER-demos/clippy.toml)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| Cargo 子命令 | [1.3 Hello Cargo](../../Book/01-getting-started/1.3-Hello-Cargo.md) |
| `as` 与 cast lint | [Item 5](../Chapter-01-Types/Item-05-type-conversions/README.md) |
| 通配符导入 | [Item 23](../Chapter-04-Dependencies/Item-23-avoid-wildcard-imports/README.md) |
| feature 否定命名 | [Item 26](../Chapter-04-Dependencies/Item-26-feature-creep/README.md) |
| `Copy` / `.clone()` | [Item 10](../Chapter-02-Traits/Item-10-standard-traits/README.md) |
| CI `-Dwarnings` | [Item 32](../Item-32-ci/README.md)（待补） |
| 工具生态 | [Item 31](../Item-31-tooling-ecosystem/README.md)（待补） |

---

## 1. 核心知识点与关键定义

### Clippy

- Rust 官方 lint 工具：`cargo clippy`（Cargo 插件）。
- 五类维度：

| 维度 | 关注点 |
|------|--------|
| **Correctness** | 常见逻辑错误 |
| **Idiom** | 不够 Rust 味 |
| **Concision** | 更简写法 |
| **Performance** | 多余分配 / 计算 |
| **Readability** | 降低理解成本 |

### 局部 / 全局豁免

```rust
#[allow(clippy::some_lint)]  // 单项
// crate 根 #![allow(...)]   // 整 crate（慎用）
```

---

## 2. 逻辑脉络

```text
Effective Rust 许多条目 = 人工 Code Review 准则
         ↓
Clippy = 编译期自动化同一套审查
         ↓
零警告基线 + CI -Dwarnings → 新退化无处藏身
         ↓
每条 lint 带文档链接 → 初学者的惯用法导师
```

本书中 Clippy 可覆盖的示例：通配符导入（Item 23）、`no_*` feature 名（Item 26）、`as` 截断（Item 5）、`clone_on_copy`（Item 10）、unsafe safety comment 等。

---

## 3. 重点结论与实用要点

### 目标：Clippy-warning free

- **修** 或 **显式 `allow`（并注释为何）** —— 二选一，别留着不管。
- 基线非零 → 新警告被噪音淹没（破窗效应）。

### 别为「误报」死磕

- 多数情况：轻微重构顺应 Clippy **比团队争论成本低**。

### CI 必跑

```bash
cargo clippy -- -Dwarnings
```

- 警告升级错误 → 不合规代码进不了 main（Item 32）。

### Pedantic 组

- 默认关的 **`clippy::pedantic`** 等极严规则 —— 可不全开，但**读 lint 说明**能加深 Rust 细节理解（如对 `as` 的 `cast_*` 系列）。

---

## 4. 案例与代码要点

### 魔法数字 → 标准库常数

```rust
pub fn circle_area(radius: f64) -> f64 {
    let pi = 3.14;
    pi * radius * radius
}
```

Clippy：`approximate value of f{32,64}::consts::PI found`  
建议：改用 **`std::f64::consts::PI`**。

### 与 ER 条目联动示例

| Lint 方向 | 关联 Item |
|-----------|-----------|
| glob re-export / wildcard | Item 23 |
| `negative_feature_names` | Item 26 |
| `cast_possible_truncation` | Item 5 |
| `clone_on_copy` | Item 10 |
| `unwrap_or_default` 等 | Item 3 |

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **破窗效应** | 几条旧 warning 常驻 → 全员无视 Clippy |
| **crate 级 `#![allow(clippy::all)]`** | 等于关掉工具；几乎 never 合理 |
| **本地不跑、只靠 CI** | 反馈太晚；开发时 `cargo clippy` 应与 `cargo check` 同级习惯 |
| **升级 toolchain 后新 lint** | 零基线才能第一时间发现 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 29](../ER-拓展索引.md#item-29)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 命令 | **`cargo clippy -- -Dwarnings`** |
| 基线 | **零警告** 或显式 allow |
| 态度 | 顺应重构 > 争误报 |
| 角色 | 机械化 Effective Rust + 学习导师 |
| Pedantic | 可不启，值得读 |
| CI | 与 Item 32 绑定 |
