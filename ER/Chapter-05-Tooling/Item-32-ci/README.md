# Item 32: Set up a continuous integration (CI) system

> **Effective Rust** · [Chapter 5 — Tooling](../ER-本书目录.md)  
> **中文**：建立持续集成 (CI) 系统  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [CI 示例](../../.github/workflows/er-study-ci.yml)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `cargo test`、测试组织 | [11.1](../../Book/11-testing/11.1-如何编写测试.md)、[11.3](../../Book/11-testing/11.3-测试的组织结构.md) |
| `Cargo.lock`（app vs lib） | [Item 25](../Chapter-04-Dependencies/Item-25-dependency-graph/README.md) |
| 工具清单 | [Item 31](../Item-31-tooling-ecosystem/README.md) |

---

## 1. 核心知识点与关键定义

### 持续集成（CI）

- 代码变更 / PR 时**自动**跑检查与测试。

### 确定性构建

- 步骤须**干净、快、确定、低误报**。
- 用 **`rust-toolchain.toml`** 钉死 toolchain：

```toml
[toolchain]
channel = "1.70.0"
# 或 channel = "nightly-2023-09-19"
```

- 避免 CI 用浮动 `stable` 而**无声**随新版本变红/变绿。

### 分级节奏（Cadence）

| 层级 | 时机 | 典型内容 |
|------|------|----------|
| 本地 / IDE | 保存 / pre-commit | fmt、rust-analyzer、clippy |
| **PR** | 每次审查 | check、test、clippy、fmt --check |
| **merge main** | 合并后 | 集成测试、多 target |
| **定期** | 日/周 | feature powerset、MSRV、无 lock 构建 |
| **后台** | 持续 | fuzz corpus 扩展（Item 30） |

---

## 2. 逻辑脉络

```text
ER 全书建议（Clippy、deny、features、doc…）
         ↓
仅文档 / 口头 → 很快腐化
         ↓
CI 自动化 → 真正防线
         ↓
流程 Bug（忘跑 codegen）→ 先加 CI 步骤再修（同 Item 30 TDD）
         ↓
全绿铁律 + 本地可复现 → 人不替机器背锅
```

---

## 3. 重点结论与实用要点

### 别浪费人类时间

- CI 步骤应能在**本地一条命令**复现 → 作者 PR 前自修，审查者看设计而非 fmt 吵架。

### 消灭 flaky tests

- 间歇失败 = **立即根因修复**或删/隔离；不能「习惯红着」。

### 绝对全绿

- 不接受「那个测试一直挂」「这个 clippy 误报不管」→ 破窗 = 新退化看不见。

### 开源 CI 安全

- 限制 **fork PR 自动跑**（或只读 token）；新贡献者 **manual approval**。
- 第三方 Action **钉 commit SHA**；写权限 step 最小化；防挖矿 / 窃 token / 供应链。

---

## 4. 案例与代码要点

### Build & Test 清单

| 检查 | 命令 / 做法 | 关联 |
|------|-------------|------|
| 全 feature | `cargo hack check --feature-powerset` | Item 26 |
| `no_std` 交叉编译 | `cargo build --target thumbv6m-none-eabi --no-default-features` | Item 33 |
| **MSRV** | `cargo +1.xx.0 check`（与 `rust-version` 一致） | Item 21 |
| 单元 + 集成 + doc | `cargo test` | Item 30 |
| Examples | `cargo test --examples` | Item 27 / 30 |
| 库无 lock 构建 | CI job 删/忽略 lock，测最新依赖 | Item 25 |
| codegen 一致 | 重跑 `prost-build` 等 → `git diff --exit-code` | — |

### Tooling 清单

```bash
cargo fmt --check
cargo clippy -- -Dwarnings          # Item 29
cargo doc --no-deps                 # Item 27 死链
cargo deny check                    # Item 25
cargo udeps                         # Item 25
cargo tarpaulin …                   # Item 30 覆盖率
```

### 自定义脚本示例

- 扫描 `TODO:` 无责任人
- 检测硬编码 secret / token
- `no_panic` crate 编译（Item 18，§6 其他 Item 曾提及）

### 参考 PR 本地复现

```bash
cargo fmt --check && cargo clippy -- -Dwarnings && cargo test --all-features
```

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **CI 里 bench 当真相** | 共享容器噪声大 → 性能基线需**专用硬件**（Item 30 / 31） |
| **fuzz 进每次 PR** | 太慢；单独 scheduled workflow |
| **只测 default features** | 漏 \(2^N\) 组合断层（Item 26） |
| **库项目只测带 lock** | 下游无 lock → 加 **lockfile-free** job |
| **toolchain 浮动** | 今天绿明天红，难归因 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 32](../ER-拓展索引.md#item-32)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 目的 | 自动化全书最佳实践 |
| 确定性 | **`rust-toolchain.toml`** |
| 节奏 | PR 快检 + 定期重检 + fuzz 后台 |
| 铁律 | **全绿**、无 flaky |
| 本地 | CI 命令 = 开发者能先跑 |
| 开源 | 限 fork CI、钉 Action SHA |
| bench | CI 结果**仅供参考** |
