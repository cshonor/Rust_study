# Item 26: Be wary of feature creep

> **Effective Rust** · [Chapter 4 — Dependencies](../ER-本书目录.md)  
> **中文**：警惕特性蔓延  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-26-feature-creep](./demo/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `Cargo.toml`、构建配置 | [1.3 Hello Cargo](../../Book/01-getting-started/1.3-Hello-Cargo.md)、[14.1 发布配置](../../Book/14-cargo-crates/14.1-采用发布配置自定义构建.md) |
| 条件编译 `#[cfg(test)]` | [11.3 测试的组织结构](../../Book/11-testing/11.3-测试的组织结构.md) |
| Feature 统一 | [Item 25](../Item-25-dependency-graph/README.md) |
| `no_std` / `std` feature | [Item 33](../Chapter-06-Beyond-Standard-Rust/Item-33-no-std/README.md)（待补） |
| CI 测 feature 组合 | [Item 32](../Chapter-05-Tooling/Item-32-ci/README.md)（待补） |
| Clippy 否定性 feature 名 | [Item 29](../Chapter-05-Tooling/Item-29-clippy/README.md)（待补） |

---

## 1. 核心知识点与关键定义

### 条件编译（`cfg` / `cfg_attr`）

- 控制项/代码块是否进入最终产物。
- 作用于 **AST**，非 C 式行级预处理器。
- 配置：`test`、`panic = "abort"` 等；部分可多值同时为真（如 32/64 位 atomics）。

### Features（Cargo）

- 在 `cfg` 之上的**包级开关**，定义于 `Cargo.toml` **`[features]`**。
- 构建时选择性编译 crate 部分功能。

### 隐式 feature

```toml
[dependencies]
serde = { version = "1", optional = true }
# 自动创建 feature "serde"
```

### Feature 统一（与 Item 25）

- 依赖图多处请求**同版本** crate 的不同 features → Cargo **取并集**，只构建一次。

---

## 2. 逻辑脉络

```text
cfg（编译器底层）
         ↓
features（Cargo 包级开关）
         ↓
全局 unification → 特性必须「纯加法」
         ↓
互斥 feature / 字段门控 → 下游无法控、必炸
         ↓
克制数量 + CI 测组合（cargo-hack）
```

---

## 3. 重点结论与实用要点

### Features 必须是加法的（Additive）

- ❌ 互斥组合（A 架构 vs B 架构）→ 别做成 feature；用 **`cfg(target_arch = "...")`** 等 target cfg。
- ❌ 否定语义（`no_std` feature 删代码）→ 用 **`std` feature 加代码**（Clippy 会 warn 否定名）。

### 不要给 pub 字段 / trait 方法加 feature 门控

- 用户无法预知 unification 是否静默开启 feature → 结构体字面量 / impl 突然缺字段或方法。

### 命名空间

- Crate **名**与 **feature 名**同空间 → 避免与常见依赖名冲突。

### 组合爆炸

- \(N\) 个独立 feature → 最多 \(2^N\) 种组合。
- **少开 feature**；CI 用 **`cargo-hack --feature-powerset`**（§6）穷举测编译。

---

## 4. 案例与代码要点

### 破坏性：pub 字段 `#[cfg(feature)]`

```rust
#[derive(Debug)]
pub struct ExposedStruct {
    pub data: Vec<u8>,
    #[cfg(feature = "schema")]
    pub schema: String,
}
```

- 用户写 `ExposedStruct { data: vec![] }` 以为 OK。
- 深层依赖开启 `schema` → unification 打开 feature → **缺少字段 `schema`**，编译失败。

### 正面范例：`std` / `alloc` / optional 依赖

```toml
[features]
default = ["std"]
std = []
# 或 optional dep 对应 feature，no_std 下不启用
```

- Item 33：`no_std` 环境靠**不启** `std`/`alloc` feature，而非 `no_std` 否定 feature。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **`default` 不是魔法** | 只是普通 feature 名；任一依赖未 `default-features = false` → unification **仍开 default**（Item 25） |
| **`no_std` 当 feature 名** | 违反加法；A 开 B 不开 → 灾难；用 **`std`** 正向开启 |
| **optional dep = 隐式 feature** | 忘记在 `[features]` 里文档化何时该开 |
| **\(2^N\) 未测** | 某组合从未 CI 构建 → 用户 первый 踩雷 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 26](../ER-拓展索引.md#item-26)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 原则 | Feature **只加能力**，互斥用 `cfg(target_*)` |
| 公开 API | **别** `#[cfg(feature)]` 门控 pub 字段/方法 |
| Unification | 全图**并集** — 用户控不了 |
| 命名 | 避 `no_*`；用 `std` 等正向名 |
| 数量 | 防 \(2^N\) — 少 feature + CI powerset |
| optional dep | 自动同名 feature |
