# Item 25: Manage your dependency graph

> **Effective Rust** · [Chapter 4 — Dependencies](../ER-本书目录.md)  
> **中文**：管理你的依赖图  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [WORKSPACE.md](../ER-demos/WORKSPACE.md) · [Cargo.toml workspace.dependencies](../ER-demos/Cargo.toml)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `Cargo.toml`、`Cargo.lock` | [1.3 Hello Cargo](../../Book/01-getting-started/1.3-Hello-Cargo.md) |
| 工作空间、共享 lock | [14.3 Cargo 工作空间](../../Book/14-cargo-crates/14.3-Cargo工作空间.md) |
| Semver、版本范围 | [Item 21](./Item-21-semver.md) |
| 公开依赖 / 多版本 | [Item 24](./Item-24-re-export-api-types.md) |
| 过程宏风险 | [Item 28](../Chapter-05-Tooling/Item-28-macros-judiciously.md)（待补） |
| CI / Dependabot / deny | [Item 32](../Chapter-05-Tooling/Item-32-ci.md)（待补） |
| FFI / ODR | [Item 34](../Chapter-06-Beyond-Standard-Rust/Item-34-ffi-boundaries.md)（待补） |

---

## 1. 核心知识点与关键定义

### 依赖图（Dependency graph）

- 程序 → 直接依赖 → 传递依赖 → 形成**依赖树/图**。

### crates.io 扁平命名空间

- Crate **名字**与 **feature 名字**共享同一命名空间（同一 crate 内）。

### 多版本共存

- Cargo 可在同一二进制图中链接同一 crate 的多个 **semver-incompatible** 版本（如 `1.2` + `3.1`）。
- **兼容**版本（如 `1.2` + `1.3`）→ 解析为**一个**最高兼容版（如 `1.3`）。

### Feature 统一（Feature unification）

- 图中多处依赖**同一版本**的 crate，但各自启用不同 features → Cargo 取**并集**，用合并后的 feature 集**只构建一次**。

---

## 2. 逻辑脉络

```text
Cargo.toml  semver 范围 → 解析 → Cargo.lock 固化
         ↓
二进制：提交 lock → 可复现构建
库：     不提交 lock（下游忽略）→ 本地/CI 可保留 lock 测上游突变
         ↓
多版本：Rust 类型隔离 OK；API 暴露则灾难（Item 24）
FFI crate：ODR → 多版本 C 符号冲突，Cargo 魔法失效
```

---

## 3. 重点结论与实用要点

### 版本声明：别太死、别太宽

| 写法 | 评价 |
|------|------|
| `"=1.2.3"` | 难收补丁、难与其他 crate 重叠 — 除非 Item 23 那种下策 |
| `"*"` / `"0.*"` | 禁；Major 突变必炸 |
| **`"1"`** 或 **`"1.4.23"`** | 推荐：允许兼容 MINOR/PATCH |

### 治理工具

| 工具 | 用途 |
|------|------|
| **`cargo tree`** | 看树、追路径、查 duplicates / features |
| **`cargo-udeps`** | 找 `Cargo.toml` 里未使用的依赖 |
| **`cargo-deny`** | CVE、许可证、重复版本策略 |

### 审慎引入新依赖

- 供应链风险：`build.rs`、过程宏（Item 28）可在**编译时**执行任意代码。
- 少依赖、常审计、CI 拦 CVE（Item 32）。

---

## 4. 案例与代码要点

### `cargo tree` 常用 flags

```bash
# 谁依赖了 problematic-crate？
cargo tree --invert problematic-crate

# 哪些 feature 被激活？
cargo tree --edges features

# 同包多版本？
cargo tree --duplicates
```

配合 Item 24：见 `rand` 重复 → 查公开 API 是否需 `pub use`。

### `Cargo.lock` 策略

| 项目类型 | lock 文件 |
|----------|-----------|
| **Binary / app** | **提交** VCS |
| **Library** | **不提交**（下游不用）；本地/CI 可保留自测 |

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **`-` vs `_`** | crate 名 `some-crate` → 代码里 `some_crate` |
| **FFI 多版本** | 纯 Rust 可多版本；含 C/C++ → **ODR 链接冲突**（Item 34） |
| **`default-features = false` 无效** | 图中任一依赖未关默认 → **unification 并集**仍开默认 feature |
| **1.x 与 1.y 不会共存** | 只有 **incompatible** 版本才双份进图 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 25](../ER-拓展索引.md#item-25)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 解析 | toml 范围 → lock 固化 |
| App lock | **要提交** |
| Lib lock | **别提交**给下游 |
| Features | 同版本取**并集** |
| 多版本 | Rust OK；**FFI 不行** |
| 工具 | tree / udeps / deny |
| 版本 | `"1"` 或 `"1.4.23"`，禁 `*` |
