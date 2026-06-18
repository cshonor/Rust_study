# 3.1 Crate Metadata（Crate 元数据）

> 所属：**Project Configuration** · [← 章索引](./README.md)

← [03 工作区](./03-workspaces.md) · 下一节 [05 构建配置](./05-build-configuration.md)

Book → [14.1 发布到 crates.io](../../00-Book/14-cargo-crates/14.1-发布到-crates-io.md) · ER → [Item 27 文档](../../01-ER/Chapter-05-Tooling/Item-27-document-public-api/README.md)

---

## 一、核心概念

**Crate Metadata** = `Cargo.toml` 里**专门用于发布到 crates.io、检索展示、合规溯源**的元信息字段。

| | |
|---|---|
| **作用** | 发布、平台展示、生态检索、开源合规 |
| **不影响** | 本地编译逻辑（与 `[profile]` 等不同） |

Workspace 内可 `[workspace.package]` 统一继承 → [03 工作区](./03-workspaces.md)

---

## 二、关键字段对照表

| 字段 | 核心用途 |
|------|----------|
| **`name`** | crates.io 唯一包名，不可重名 |
| **`version`** | SemVer 语义化版本 → [07 MSRV](./07-msrv.md) · [08 依赖下界](./08-minimal-dependency-versions.md) |
| **`description`** | 一句话摘要 — 搜索列表、首页 |
| **`documentation`** | 自定义文档 URL；缺省 → docs.rs |
| **`license`** | 开源许可证（MIT / Apache-2.0 等）— **发布必填** |
| **`repository`** | 源码仓库（GitHub 等）— 溯源 |
| **`keywords`** | 检索标签（**最多 5 个**） |
| **`categories`** | crates.io **官方固定分类** |
| **`readme`** | README 路径 — crate 主页展示 |
| **`include`** | **白名单** — 仅打包列表内文件 |
| **`exclude`** | **黑名单** — 排除指定路径 |

---

## 三、实操规范

### 1. 发布前检查打包内容

```bash
cargo package --list
```

查看打进发布 tarball 的文件 — 防冗余 / 敏感文件上传。

### 2. 过滤敏感与无用文件

| 方式 | 用法 |
|------|------|
| **`exclude`** | `exclude = [".env", "tests/local_config.toml", "secret/"]` |
| **`.cargo_vcsignore`** | 语法同 `.gitignore`，**专管发布打包**，与 git 忽略分离 |

简单场景用 `exclude`；规则多时用 `.cargo_vcsignore`。

### 3. 完整示例模板

```toml
[package]
name = "demo-lib"
version = "0.1.0"
description = "A simple demo library for learning cargo metadata"
documentation = "https://docs.rs/demo-lib"
license = "MIT OR Apache-2.0"
repository = "https://github.com/xxx/demo-lib"
keywords = ["rust", "demo", "util"]
categories = ["development-tools"]
readme = "README.md"
exclude = [".env", "tmp/", "private_notes.md"]
```

> SPDX 双许可常用 `MIT OR Apache-2.0`（或 `MIT/Apache-2.0` 旧写法，以 Cargo 文档为准）。

---

## 四、配套参考

| 资源 | 说明 |
|------|------|
| **Book 14.1** | 登录、`cargo publish`、权限 |
| **ER Item 27** | metadata + 文档是库作者硬性标准 — 缺 description / readme / license 降低可信度 |
| **SemVer** | [ER Item 21](../../01-ER/Chapter-04-Dependencies/Item-21-semver/README.md) |

---

## 五、核心速记

1. Metadata **服务** crates.io 展示、检索、合规 — 不驱动编译。  
2. **`name` + `version`** = crate 身份；**`license` + `repository`** = 开源必备。  
3. **`include` / `exclude`** 控体积与隐私。  
4. 发布前 **`cargo package --list`** 必跑。

→ 速记：[04-cheat-sheet.md](./04-cheat-sheet.md) · 下一节：[05 构建配置](./05-build-configuration.md)
