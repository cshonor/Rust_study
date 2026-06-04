# Rust_study

Rust 学习笔记 + 每章 demo。

- **主线**：《Rust 程序设计语言》（The Book），章节在 `Book/01-*`～`Book/19-*`，导航 [`Book/Book-本书目录.md`](Book/Book-本书目录.md)。
- **扩展**：[Effective Rust](https://www.effective-rust.com/)（David Drysdale，35 条建议），本地索引见 [`ER/ER-本书目录.md`](ER/ER-本书目录.md)。
- **网络（UNP）**：[UNIX 网络编程](UNP/UNP-本书目录.md) 卷 1 精读（HFT 向），与 Rust 主线并行。

## 目录结构

- **Book（The Book）**：[`Book/Book-本书目录.md`](Book/Book-本书目录.md) + `Book/01-*`～`Book/19-*`（笔记与 demo）
- **ER（Effective Rust）**：`ER/Chapter-01-Types/` … `Chapter-06-Beyond-Standard-Rust/`，每条 Item 一个 `Item-NN-*.md` 占位（与 The Book 并行，按需填写）
- **UNP**：[`UNP/UNP-本书目录.md`](UNP/UNP-本书目录.md) + `UNP/Vol1/`（网络编程精读，HFT 批注）
- **笔记**：每章/小节为一个 `*.md`
- **demo**：从第 3 章起，按规则 **“一个 md 对应一个独立 Cargo project”**
  - 例：`Book/03-common-concepts/3.3-函数.md` ↔ `Book/03-common-concepts/3.3-functions-demo/`

## Rust 工具链：Stable / Nightly / Edition

Rust 基础概念，与具体 demo 无关；本仓库 CI 踩坑也源于此。

### 核心一句话

**源码不变，选哪个 `rustc` 编译，就遵循哪套规则。** Nightly 编译器跟 `main` 最新规则；Stable 编译器用发布日冻结的规则。

> 源码 = 你的答卷；`rustc`（Stable / Nightly）= 不同版本的阅卷官 —— 语法都能认，但「哪些 API 算分、哪些写法判错」不一样。

### 仓库与打包（只有一条开发线）

Rust **没有** Stable / Nightly 两条并行开发分支，只有 **`rust-lang/rust` 的 `main`**：

| Channel | 含义 |
|---------|------|
| **Nightly** | 每天从 `main` 打包的 `nightly-rustc` + 当日 std |
| **Beta** | 每 6 周从 Nightly 切出的候选版，功能冻结 |
| **Stable** | Beta 打磨后发布；**代码均来自过往某次 Nightly** |

口诀：**源码同根，打包分时；Nightly 追新，Stable 锁稳。**

### 写代码 → 编译全链路

1. **写 `.rs`**：只写 Edition 定义的语法（`fn` / `struct` / `unsafe` / `&mut` …），与 Stable/Nightly **无直接关系**。
2. **rustup 装多版编译器**：本机可同时有 `stable` / `beta` / `nightly` 三份 `rustc`。
3. **决定性动作：用哪份 `rustc` 编译**：

```bash
cargo +stable build    # 稳定规则：禁止 #![feature(...)]
cargo +nightly build   # 可开不稳定 feature、-Z 选项
```

### 编译时 Stable / Nightly 三处不同

| 维度 | Nightly `rustc` | Stable `rustc` |
|------|-----------------|----------------|
| **不稳定 Feature** | `#![feature(xxx)]` 可启用试验 API | **未稳定 feature 的实现已移除**，写 `#![feature(...)]` 直接报错 |
| **Unsafe / UB 规则** | 跟进 `main` 最新内存模型、别名规则 | 冻结在发布日，不随后续 `main` 改动更新 |
| **标准库 std** | 每日更新的 std 产物 | 发布时冻结的 std；新 API 旧 Stable 没有 |

### Edition ≠ Channel（避混淆）

| 分类 | 管什么 | 受 Stable/Nightly 影响？ |
|------|--------|--------------------------|
| **Edition**（2018 / 2021 / 2024） | 基础语法怎么写（模块路径、`dyn`、迁移 lint …） | **否** — 同一 `rustc` 可编多 Edition，`Cargo.toml` 里 `edition = "2021"` 切换 |
| **Channel**（Stable / Nightly） | 能调哪些 API、能否开 feature、检查逻辑、std 版本 | **是** |

> Edition 管**语法拼写**；Stable/Nightly 管**可用 API、不稳定 Feature、UB 规则、编译器检查逻辑**。

### 示例：Stable 语法，仅 Nightly 可用

**源码相同**，换编译器结果不同：

```rust
// demo.rs —— Edition 2021 标准写法，无语法问题
#![feature(try_blocks)]

fn demo() -> Option<i32> {
    try { Some(1? + 2) }
}
```

```bash
rustc +stable  demo.rs   # error: feature `try_blocks` is not stable
rustc +nightly demo.rs   # 通过（若 nightly 仍保留该 feature）
```

不改源码、但工具链必须是 Nightly 的常见情况（`-Z` / 组件仅随 nightly 分发）：

```bash
cargo +stable doc -Z unstable-options --output-format json   # 失败：-Z 仅 nightly
cargo +nightly doc -Z unstable-options --output-format json  # 成功
```

### Nomicon 与编译器对应

| 文档 | 配套编译器 | 特点 |
|------|------------|------|
| **The Rustonomicon**（Stable 语境） | `+stable` | 不加 `feature`，可上生产的 unsafe 边界 |
| **跟踪 `main` 的 unsafe 草案** | `+nightly` | 含试验性规则；前沿写法需 nightly 才编得过 |

### 实操建议

| 场景 | 命令 | 对照 |
|------|------|------|
| 学原理、试底层 | `cargo +nightly build` | Nomicon + 不稳定 feature |
| 项目落地 / CI 默认 | `cargo +stable build` | 去掉 `#![feature(...)]`，换稳定 API |

本仓库 ER-demos 的 MSRV、CI nightly job 等落地细节见 [`ER/ER-demos/WORKSPACE.md`](ER/ER-demos/WORKSPACE.md)。

The Book 对应章节：[附录索引](Book/appendix/README.md)（A～G）；工具链详解见 [G.7 Nightly Rust](Book/appendix/G.7-Nightly-Rust与发布渠道.md)。

## 快速开始

### 运行某个 demo（The Book）

进入对应 demo 目录后运行：

```bash
cargo run
```

示例（函数章节 3.3）：

```bash
cd Book/03-common-concepts/3.3-functions-demo
cargo run
```

### 猜数字游戏

```bash
cd Book/02-guessing-game
cargo run
```

### 第 19 章「高级特性」部分 demo（The Book）

```bash
cd Book/19-advanced-features/19.2-advanced-traits-demo
cargo run
```

过程宏 workspace 示例：

```bash
cd Book/19-advanced-features/19.5-macros-demo
cargo run -p decl_macro_demo
cargo run -p pancakes
```

## 说明

- 章节目录使用英文命名，避免 Windows 终端乱码。
- `target/` 已在 `.gitignore` 中忽略。
- **VSCode + rust-analyzer**：见 [`docs/rust-analyzer-VSCode配置.md`](docs/rust-analyzer-VSCode配置.md)；仓库内 [`.vscode/settings.json`](.vscode/settings.json) 为开箱即用工作区配置。
