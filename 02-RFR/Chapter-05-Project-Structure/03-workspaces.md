# 2. Workspaces（工作区）

> 所属：**Workspaces** · [← 章索引](./README.md)

← [02 crate 内使用 Feature](./02-using-features-in-crate.md) · 下一节 [04 Crate 元数据](./04-crate-metadata.md)

前置 → [00 Package vs Workspace](./00-package-vs-workspace.md)（单包 lib+bin · 多包辨析）

Book → [14.3 Cargo 工作空间](../../00-Book/14-cargo-crates/14.3-Cargo工作空间.md) · demo → [14.3-workspace-demo](../../00-Book/14-cargo-crates/14.3-workspace-demo/) · [14.3-hft-workspace-demo](../../00-Book/14-cargo-crates/14.3-hft-workspace-demo/)

---

## 一、核心作用：解决巨石单 crate 痛点

### 巨石单 Package 缺陷

单 Package 内 `lib` 收拢所有 `mod` — 改任意内部模块常触发 **整个 lib 重编**；边界仅 `pub` 可见性，无 crate 级隔离。  
→ 辨析：[00 Package vs Workspace](./00-package-vs-workspace.md)

### Workspace 价值

同一仓库拆成多个独立 **member crate**：

| 收益 | 说明 |
|------|------|
| **增量编译** | 只重编变更 crate + 依赖它的上层 |
| **边界清晰** | 库 / 二进制 / 工具分包 |
| **统一治理** | 依赖、构建、版本锁定 — 适合 monorepo |

---

## 二、根目录 `[workspace]` 标准配置

### 1. 虚拟工作区（最常用）

根 `Cargo.toml` **只有** `[workspace]`，无 `[package]`：

```toml
[workspace]
members = ["crates/foo", "crates/bar", "cli"]
exclude = ["crates/temp_demo"]
resolver = "2"
```

### 2. 带包工作区

同时存在 `[package]` + `[workspace]` — 根本身也是可编译 crate，并管理成员。

### 关键字段

| 字段 | 作用 |
|------|------|
| **`members`** | 成员 crate 路径；支持 `crates/*` 通配 |
| **`exclude`** | 排除路径，不纳入工作区 |
| **`resolver = "2"`** | 现代 feature 解析（见 §五） |
| **`default-members`** | 无 `-p` 时默认操作的包列表 |

---

## 三、全局共享机制

### 1. 统一 `Cargo.lock`

- **仅根目录**一份 lock，所有成员共用  
- 同一第三方依赖全工作区 **同一版本** — 杜绝多副本冲突  
- 子 crate **不再**独立 lock  

### 2. 共享根 `target/`

- 编译产物、增量缓存、依赖库统一输出到根 `target/`  
- 复用缓存 · 根目录 `cargo clean` 一次清完  

### 3. 仅根生效的配置

子 crate 内写了**会被忽略**：

| 配置 | 用途 |
|------|------|
| **`[profile.*]`** | dev / release / opt-level 等 |
| **`[patch]` / `[replace]`** | 依赖替换、本地补丁 |
| **`[workspace.dependencies]`** | 全局依赖版本 |

```toml
# 根 Cargo.toml
[workspace.dependencies]
tokio = { version = "1.0", features = ["macros"] }
serde = { version = "1.0", features = ["derive"] }

# 子 crate
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
```

### 4. `[workspace.package]` 元数据复用

统一 version / authors / license / edition — 子 crate `workspace = true` 继承。

---

## 四、工作流与常用命令

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

### 成员互相依赖

```toml
# crates/bar/Cargo.toml
[dependencies]
foo = { path = "../foo", version = "0.1" }
```

工作区内 path 依赖自动识别为 member，不走 registry。

→ 路径：[7.3.1 跨 Package 与 Workspace](../../00-Book/07-packages-modules/7.3.1-跨Package路径与Workspace依赖.md)

---

## 五、`resolver = "2"`（必考）

1. **区分** 普通依赖 / build 依赖 / proc-macro — feature **不**全局乱合并  
2. **平台条件依赖隔离** — Windows 专属 feature 在 Linux 构建时不被强开  
3. 2021 edition 单 crate 默认 resolver 2；**虚拟工作区须手动声明**，否则仍用 v1  

→ 与 Feature 统一：[01 Feature](./01-defining-including-features.md) · [02 库内使用](./02-using-features-in-crate.md)

---

## 六、推荐目录结构

```text
project-root/
├── Cargo.toml          # [workspace]
├── Cargo.lock          # 全局唯一
├── target/             # 统一产物
├── crates/
│   ├── foo-lib/
│   └── bar-core/
└── cli/                # 二进制
```

---

## 七、对照阅读

| 资源 | 路径 |
|------|------|
| Book 14.3 | [Cargo 工作空间](../../00-Book/14-cargo-crates/14.3-Cargo工作空间.md) |
| 入门 demo | [14.3-workspace-demo](../../00-Book/14-cargo-crates/14.3-workspace-demo/) |
| HFT 多包示例 | [14.3-hft-workspace-demo](../../00-Book/14-cargo-crates/14.3-hft-workspace-demo/) |
| ER 多 crate | [ER-demos WORKSPACE](../../01-ER/ER-demos/WORKSPACE.md) |

---

## 八、核心速记

1. **目的**：拆巨石 crate → 增量编译、解耦。  
2. **两大共享**：根 `Cargo.lock` · 根 `target/`。  
3. **根独占**：profile / patch / `[workspace.dependencies]`。  
4. **`resolver = "2"`** — 隔离依赖类型 feature。  
5. **`--workspace`** 全局 · **`-p`** 单成员。  
6. 子包 `workspace = true` 复用全局依赖版本。

→ 速记：[03-cheat-sheet.md](./03-cheat-sheet.md) · 下一节：[04 Crate 元数据](./04-crate-metadata.md)
