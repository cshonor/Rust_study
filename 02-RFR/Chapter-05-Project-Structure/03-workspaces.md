# 2. Workspaces（工作区）

> 所属：**Workspaces** · [← 章索引](./README.md)

← [02 crate 内使用 Feature](./02-using-features-in-crate.md) · 下一节 [04 Crate 元数据](./04-crate-metadata.md)

前置 → [00 Package vs Workspace](./00-package-vs-workspace.md)（单包 lib+bin · 多包辨析）

Book → [14.3 Cargo 工作空间](../../00-Book/14-cargo-crates/14.3-Cargo工作空间.md) · demo → [14.3-workspace-demo](../../00-Book/14-cargo-crates/14.3-workspace-demo/) · [14.3-hft-workspace-demo](../../00-Book/14-cargo-crates/14.3-hft-workspace-demo/)

---

## 核心结论

1. **根 `Cargo.toml` 不负责业务依赖、不负责子包版本号** — 只管「有哪些子包」；第三方依赖约束、包自身 `version`，**各子包自己的 `Cargo.toml` 独立管控**。  
2. **Workspace 统一落地的一件事**：根目录**唯一** `Cargo.lock` — 全工作区同一第三方 crate **只锁一个版本**，不会多版本并存。

```text
子包 TOML 提「版本要求」  →  Cargo 解析  →  lock 写「全局落地版本」
```

---

## 一、核心作用：解决巨石单 crate 痛点

### 巨石单 Package 缺陷

单 Package 内 `lib` 收拢所有 `mod` — 改任意内部模块常触发 **整个 lib 重编**；边界仅 `pub` 可见性，无 crate 级隔离。  
→ 辨析：[00 Package vs Workspace](./00-package-vs-workspace.md)

### Workspace 价值

同一仓库拆成多个独立 **member Package**：

| 收益 | 说明 |
|------|------|
| **增量编译** | 只重编变更 crate + 依赖它的上层 |
| **边界清晰** | 库 / 二进制 / 工具分包 |
| **统一治理** | lock、构建缓存 — 适合 monorepo |

---

## 二、根 `Cargo.toml` 写什么？能干什么？

根文件通常**没有** `[package]`、`[dependencies]`，核心只有 `[workspace]`：

```toml
# 根目录 Cargo.toml
[workspace]
members = [
    "crates/*",
    "apps/*",
]
exclude = []                              # 可选：排除某些路径
default-members = ["apps/server-app"]     # 可选：无 -p 时默认构建谁
resolver = "2"                            # 虚拟工作区建议手写，见 §七
```

### ✅ 根能干

| 职责 | 说明 |
|------|------|
| **登记成员** | `members` / `exclude` — Cargo 知道哪些子目录属于工作区 |
| **统一 lock** | 根目录唯一 `Cargo.lock`，锁定全工作区第三方版本 |
| **批量命令** | 根目录 `cargo build` / `test` / `clippy` 可操作全部或 `default-members` |
| **可选集中配置** | `[workspace.dependencies]`、`[workspace.package]`、`profile`、`patch`（见 §五） |

### ❌ 根不能干

| 误区 | 正解 |
|------|------|
| 在根写 `[package]` 版本、名字给「整个项目」 | **每个子包**各自 `[package].name` / `version` |
| 根写 `[dependencies]` 就自动注入所有子包 | **不能** — 子包须在自己 TOML 声明需要什么；根只能提供 `workspace.dependencies` **供继承** |
| 根统一管控子包 feature、内部编译选项 | 各子包自己的 `[features]`、`[lib]` 等 |

---

## 三、版本号与依赖：每个子包自己管

每个 member（如 `crates/core-utils`、`apps/server-app`）都是**独立 Package**：

```toml
# crates/core-utils/Cargo.toml
[package]
name = "core-utils"
version = "0.1.0"    # 独立迭代，改这里不影响别的子包
edition = "2021"

[dependencies]
tokio = "1.35"
serde = { version = "1.0", features = ["derive"] }
```

| 字段 | 谁管 | 说明 |
|------|------|------|
| **`[package].version`** | 子包自己 | 工具库升版只改自己 |
| **`[dependencies]`** | 子包自己 | 要什么第三方、约束范围、开哪些 feature |
| **工作区内依赖** | 子包 path | `core-utils = { path = "../../crates/core-utils", version = "0.1.0" }` |

```toml
# apps/server-app/Cargo.toml
[dependencies]
core-utils = { path = "../../crates/core-utils", version = "0.1.0" }
axum = "0.7"
```

> `path` 依赖发布到 crates.io 时须带 `version`（与 path 包版本兼容），见 [04 元数据](./04-crate-metadata.md)。

---

## 四、`Cargo.lock` 如何统一第三方版本？

**不靠**根 TOML 写死版本，靠**全局唯一 lock**：

1. 子包 A：`tokio = ">=1.30"`；子包 B：`tokio = ">=1.32"`  
2. 整体构建时 Cargo 解析出**同时满足所有约束的最高兼容版本**  
3. 写入根目录 `Cargo.lock` — 全工作区编译时**都用这一个 tokio**  
4. 各子包 TOML 仍保留自己的**语义化约束**；lock 负责**收敛落地**

```text
子包提要求 → lock 统一落地 → 不会出现同一 crate 多版本并存
```

### 与 `target/` 共享

- 编译产物、增量缓存统一输出到根 `target/`  
- 根 `cargo clean` 一次清完  

---

## 五、公共依赖复用：`[workspace.dependencies]`

没有「根 TOML 一键注入全局依赖」，但有**规范集中写法** — 版本在根定义一次，子包**主动继承**：

### 1. 根定义工作区依赖

```toml
[workspace]
members = ["crates/*", "apps/*"]

[workspace.dependencies]
tokio = "1.35.0"
serde = { version = "1.0", features = ["derive"] }
axum = "0.7"
```

### 2. 子包继承，不必重复写版本

```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
```

| 好处 | 说明 |
|------|------|
| 一处维护版本 | 改根 `workspace.dependencies`，所有 `workspace = true` 的子包同步 |
| 仍是子包拥有依赖 | 只是版本从工作区读取，**不是**根强制注入 |

### `[workspace.package]` 元数据同理

根统一 `version` / `authors` / `license` / `edition`，子包 `edition = { workspace = true }` 继承 — 见 [04 元数据](./04-crate-metadata.md)。

### 两种工程化模式

| 模式 | 写法 | 特点 |
|------|------|------|
| **经典** | 各子包 TOML 各自写依赖版本 | lock 仍全局统一；重复多 |
| **现代** | 根 `[workspace.dependencies]` + 子包 `workspace = true` | 版本一处维护，更整洁 |

### 仅根生效、子包写了会被忽略的配置

| 配置 | 用途 |
|------|------|
| **`[profile.*]`** | dev / release / opt-level |
| **`[patch]` / `[replace]`** | 依赖替换、本地补丁 |

→ patch / profile 详 [05 构建配置](./05-build-configuration.md)

---

## 六、工作流与常用命令

### 全局批量（根目录）

```bash
cargo check --workspace
cargo test --workspace
cargo build --release --workspace
cargo doc --workspace
```

### 单独操作某成员

```bash
cargo build -p foo
cd crates/foo && cargo run   # 自动识别上层 workspace
```

### 增量编译逻辑

修改 `crates/foo`：

1. 仅重编译 `foo`  
2. 直接依赖 `foo` 的上层 crate 重新链接  
3. 不依赖 `foo` 的包 **不参与**编译  

---

## 七、`resolver = "2"`（必考）

1. **区分** 普通依赖 / build 依赖 / proc-macro — feature **不**全局乱合并  
2. **平台条件依赖隔离** — Windows 专属 feature 在 Linux 构建时不被强开  
3. 2021 edition 单 crate 默认 resolver 2；**虚拟工作区须手动声明**，否则仍用 v1  

→ 与 Feature 统一：[01 Feature](./01-defining-including-features.md) · [02 库内使用](./02-using-features-in-crate.md)

---

## 八、推荐目录结构

```text
project-root/
├── Cargo.toml          # [workspace] · 可选 workspace.dependencies
├── Cargo.lock          # 全局唯一
├── target/             # 统一产物
├── crates/
│   ├── core-utils/
│   └── database/
└── apps/
    └── server-app/     # bin 入口
```

---

## 九、整体职责一览

| 维度 | 谁负责 |
|------|--------|
| **成员列表** | 根 `[workspace].members` |
| **子包版本号** | 各子包 `[package].version` |
| **依赖声明** | 各子包 `[dependencies]`；版本可继承 `workspace.dependencies` |
| **最终锁定版本** | 根 `Cargo.lock` |
| **根 TOML 本质** | 管成员 + 可选集中版本/元数据；**不属于任何一个 crate** |

---

## 十、对照阅读

| 资源 | 路径 |
|------|------|
| Book 14.3 | [Cargo 工作空间](../../00-Book/14-cargo-crates/14.3-Cargo工作空间.md) |
| 入门 demo | [14.3-workspace-demo](../../00-Book/14-cargo-crates/14.3-workspace-demo/) |
| HFT 多包示例 | [14.3-hft-workspace-demo](../../00-Book/14-cargo-crates/14.3-hft-workspace-demo/) |
| ER 多 crate | [ER-demos WORKSPACE](../../01-ER/ER-demos/WORKSPACE.md) |

---

## 十一、核心速记

1. **根不管子包 version** — 只管 members；**lock 统一第三方落地版本**。  
2. **子包各自声明依赖**；`workspace.dependencies` = 版本集中维护 + 子包 `workspace = true` 继承。  
3. **两大共享**：根 `Cargo.lock` · 根 `target/`。  
4. **`resolver = "2"`** — 虚拟工作区须手写。  
5. **`--workspace`** 全局 · **`-p`** 单成员。

→ 速记：[03-cheat-sheet.md](./03-cheat-sheet.md) · 下一节：[04 Crate 元数据](./04-crate-metadata.md)
