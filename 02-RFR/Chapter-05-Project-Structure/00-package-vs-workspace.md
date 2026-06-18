# 0. Package vs Workspace（单包 lib+bin 与多包工作区）

> 第 5 章基础辨析 · [← 章索引](./README.md) · 下一节 [01 Feature](./01-defining-including-features.md)

前置 → Book [1.3 Hello Cargo](../../00-Book/01-getting-started/1.3-Hello-Cargo.md) · [7.3.1 跨 Package 路径](../../00-Book/07-packages-modules/7.3.1-跨Package路径与Workspace依赖.md)

---

## 误区纠正

| 误区 | 正解 |
|------|------|
| 一个 Package 只能一个 crate | **一个 Package 可同时有 lib + bin（2 个 crate）** |
| `cargo new` 只有一个 crate | 默认只有 **bin**；加 `src/lib.rs` 后即有 **lib + bin** |
| 文件夹 + `mod.rs` = 独立 crate | 单 Package 内只是 **mod 模块**，不是独立 crate |
| lib 收拢 mod = Workspace | 仍是 **单 Package**；Workspace = 多个独立 Package |

---

## 一、单 Package：lib crate + bin crate

`cargo new myproj` → 默认仅 `src/main.rs`（**1 个二进制 crate**）。

手动添加 `src/lib.rs` 后，**同一 Package** 包含：

| Crate | 入口 | 产物 |
|-------|------|------|
| **库 crate** | `src/lib.rs` | `.rlib` 等 — 可被 main / 外部依赖 |
| **二进制 crate** | `src/main.rs` | `.exe` — 程序入口 |

### 关键规则

`main.rs` 可直接 `use myproj::xxx;` — **无需**在 `Cargo.toml` 为同包 lib 写依赖，Cargo 自动关联。

**实践**：lib 收拢业务模块；main 只做入口、参数解析、初始化 — **仍是单 Package，不是 Workspace**。

### 目录示例

```text
myproj/
├── Cargo.toml          # 唯一 Package 配置
└── src/
    ├── lib.rs          # 库 crate 根：pub mod db; pub mod utils;
    ├── main.rs         # 二进制 crate 入口
    ├── db/
    │   ├── mod.rs
    │   └── mysql.rs
    └── utils/
        ├── mod.rs
        └── crypto.rs
```

| 特点 | 说明 |
|------|------|
| 一份 `Cargo.toml` | 版本、依赖、feature **全局统一** |
| `db/`、`utils/` | **同一 lib crate 内的 mod**，非独立 crate |
| 编译 | lib 先编，再链进 main；改 `utils` → **整个 lib 重编**（巨石倾向） |

---

## 二、Workspace：多个独立 Package

顶层 **根 `Cargo.toml`（`[workspace]`）** + 子目录各 **独立 Package**（各自 `Cargo.toml`、版本、依赖、crate）。

### 标准目录

```text
my_workspace/
├── Cargo.toml               # workspace 根：members，不写项目依赖
├── Cargo.lock               # 全局唯一
├── crates/
│   ├── core-utils/          # Package 1 · lib
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── database/            # Package 2
│   └── business-logic/      # Package 3
└── apps/
    └── server-app/          # Package 4 · bin 入口
        ├── Cargo.toml
        └── src/main.rs
```

### 根 `Cargo.toml`

```toml
[workspace]
members = [
    "crates/core-utils",
    "crates/database",
    "crates/business-logic",
    "apps/server-app",
]
```

根文件**不写** `[package]` 依赖 / version（虚拟工作区）— 只管成员列表。

### 子包 `crates/core-utils/Cargo.toml`

```toml
[package]
name = "core-utils"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

### 入口 `apps/server-app/Cargo.toml`

```toml
[package]
name = "server-app"
version = "0.1.0"
edition = "2021"

[dependencies]
core-utils = { path = "../../crates/core-utils" }
database = { path = "../../crates/database" }
business-logic = { path = "../../crates/business-logic" }
tokio = "1.0"
```

→ 详 [03 工作区](./03-workspaces.md)（lock · target · resolver 2）

---

## 三、架构 A vs B 对比

| | **A：单 Package（lib + main）** | **B：Workspace 多 Package** |
|---|--------------------------------|------------------------------|
| Package 数 | **1** | **多个** |
| Crate | lib + bin（同包） | 每 Package 自有 lib/bin |
| `db/` 文件夹 | lib 内 **mod** | 常做成**独立 Package** |
| `Cargo.toml` | **一份** | 每 Package 一份 |
| 编译 | 改模块 → 常重编**整个 lib** | 只重编**变更的子包** |
| 边界 | `pub` 可见性 | **crate** 级强隔离 |
| 适合 | 小项目、工具、几千行 | 分层架构、长期维护、大型 monorepo |

---

## 四、`lib.rs` 收拢 mod 的两层含义

| 场景 | `lib.rs` 作用 |
|------|----------------|
| **单 Package** | `pub mod db;` — `db` 是**同一 lib crate 内模块**；main 用 `use myproj::db` |
| **Workspace** | `db` 做成**独立子 crate**（自带 `Cargo.toml`），不再是大 lib 里的文件夹 |

---

## 五、一句话速记

- **一个 `Cargo.toml` = 一个 Package**；一个 Package 可有 **多个 crate**（lib / bin）。  
- **Workspace** = 多 Package 同仓，根 `[workspace]` 统一管理。  
- **文件夹 + `mod.rs`** → 单 crate 内模块；**文件夹 + `Cargo.toml`** → 独立 Package / crate。

→ 速记：[00-cheat-sheet.md](./00-cheat-sheet.md) · 工作区配置：[03](./03-workspaces.md)
