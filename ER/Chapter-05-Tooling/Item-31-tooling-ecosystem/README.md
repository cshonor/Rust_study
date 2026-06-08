# Item 31: Take advantage of the tooling ecosystem

> **Effective Rust** · [Chapter 5 — Tooling](../ER-本书目录.md)  
> **中文**：充分利用工具生态系统  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `rustup`、安装 | [1.1 安装](../../Book/01-getting-started/1.1-安装.md) |
| `cargo` 基础 | [1.3 Hello Cargo](../../Book/01-getting-started/1.3-Hello-Cargo.md) |
| 发布、doc | [14.2 Crates.io](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| CI 集成 | [Item 32](../Item-32-ci/README.md)（待补） |

---

## 1. 核心知识点与关键定义

### 工具生态

- 现代语言 = 编译器 + **维护与质量**配套基础设施。

### 基础环境

| 工具 | 作用 |
|------|------|
| **`cargo`** | 依赖、构建、测试、doc… |
| **`rustup`** | 安装 / 切换 toolchain |
| **rust-analyzer** | IDE 补全、跳转、诊断 |
| **[Rust Playground](https://play.rust-lang.org/)** | 在线试语法、分享片段 |

### 内置 Cargo 子命令（常用）

| 命令 | 作用 |
|------|------|
| `cargo fmt` | 格式化 |
| `cargo check` | 只类型检查，不产二进制（快） |
| `cargo clippy` | Lint（Item 29） |
| `cargo doc` | API 文档（Item 27） |
| `cargo bench` | 基准（Item 30） |
| `cargo update` | 升级 lock 内依赖 |
| `cargo tree` | 依赖树（Item 25） |
| `cargo metadata` | 工作空间 / 依赖 **JSON 元数据** |

---

## 2. 逻辑脉络

```text
cargo metadata (JSON)
         ↓
cargo_metadata crate → udeps / deny / …
         ↓
syn / quote / AST 级工具 → expand / tarpaulin / …
         ↓
日常痛点 → 搜 cargo-<name> → 有价值则 CI（Item 32）
         ↓
快且无 FP 的（rustfmt）→ 编辑器 save hook
```

---

## 3. 重点结论与实用要点

### 别只 `build` / `test`

- 遇到痛点先搜 **`cargo-*`**；维护度参差，引入前看活跃度。

### 有价值 → 进 CI（Item 32）

- Clippy、deny、fmt check、semver-checks… 自动化。

### 左移到编辑器

- **`rustfmt` on save** — 零误报、极快，收益最大。

---

## 4. 案例与代码要点

### 全书高频工具速查

| 领域 | 工具 | ER |
|------|------|-----|
| **unsafe / UB** | **Miri** | Item 16 |
| **依赖升级** | **Dependabot** / Renovate | Item 21 / 25 |
| **Semver break** | **cargo-semver-checks** | Item 21 |
| **未用依赖** | **cargo-udeps** | Item 25 |
| **CVE / license** | **cargo-deny** | Item 25 |
| **宏展开** | **cargo-expand** | Item 28 |
| **Lint** | **Clippy** | Item 29 |
| **汇编** | **Godbolt** | Item 30 |
| **Fuzz** | **cargo-fuzz** | Item 30 |
| **Bench** | **criterion** | Item 30 |
| **覆盖率** | **cargo-tarpaulin** / llvm-cov | Item 30 |
| **FFI 绑定** | **bindgen** | Item 35 |
| **Feature 穷举** | **cargo-hack** | Item 26 |

### `cargo metadata` 示例

```bash
cargo metadata --format-version 1 | jq '.packages[].name'
```

第三方工具读此 JSON，无需解析 `Cargo.toml` 文本。

### `cargo-expand`

```bash
cargo install cargo-expand
cargo expand --lib my_module::some_macro_user
```

调试 Item 28 复杂宏时必备。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **废弃的 cargo 插件** | 看 crates.io 更新日期、Rust 版本兼容 |
| **企业 / 嵌入式定制链** | 无 `rustup`、无原生 `cargo`（如 Android Soong）→ 工具链与文档不同 |
| **只本地用不进 CI** | fmt/clippy 未 CI 化 → 主分支仍可 merge 脏代码 |
| **堆太多慢工具进 PR** | fuzz / 全 feature powerset 应定时 job（Item 30） |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 31](../ER-拓展索引.md#item-31)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 日常三件套 | **check / fmt / clippy** |
| 元数据 | **`cargo metadata`** 驱动生态工具 |
| 宏 | **`cargo-expand`** |
| 依赖 | **tree / udeps / deny** |
| 采纳 | 好用 → **CI**；fmt → **编辑器** |
| 选型 | 看维护度，别绑死废弃插件 |
