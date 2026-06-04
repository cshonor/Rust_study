# Item 21：MSRV 与 `rust-version`

本 workspace 根 `Cargo.toml` 声明：

```toml
[workspace.package]
rust-version = "1.70"
```

CI 见 [er-study-ci.yml](../../../.github/workflows/er-study-ci.yml) 的 `msrv` job。

本地验证：

```bash
rustup toolchain install 1.70.0
cargo +1.70.0 check --workspace
```

## Rust 工具链：源码同根，打包分时

Rust **没有** Stable / Nightly 两条并行开发分支，只有 **`rust-lang/rust` 的 `main` 一条主干**：

| Channel | 含义 |
|---------|------|
| **Nightly** | 每天从 `main` 打包的快照，含尚在试验的 `feature(xxx)` |
| **Beta** | 每 6 周从 Nightly 切出的发布候选，功能冻结 |
| **Stable** | Beta 打磨完成后发布；**代码均来自过往某次 Nightly** |

口诀：**源码同根，打包分时；Nightly 追新，Stable 锁稳。**

### 语法 vs 工具链能力（CI 踩坑根源）

- **Edition（2018/2021）**：Stable 与 Nightly **共用同一套语法**（`fn`、`struct`、`unsafe` 等）。
- **稳定 API**：走完 stabilization 周期后进入 Stable，任意 Stable 版本可用。
- **夜间专属能力**：仍在 `main` 上试验、尚未转正 —— 可能需 `#![feature(…)]`，或 **编译器/工具链 `-Z` 选项**；Stable 直接报错。

示例 —— 语法是标准 Rust，但 **Stable 不可用**：

```rust
#![feature(try_blocks)] // Stable：error: feature 未稳定

fn demo() -> Option<i32> {
    try { Some(1? + 2) }
}
```

示例 —— 本仓库 CI 里更常见的情况（**无需改源码**，但工具链必须是 Nightly）：

```bash
# rustdoc JSON：Stable 报 -Z 仅 nightly 可用
cargo +stable doc -Z unstable-options --output-format json   # 失败
cargo +nightly doc -Z unstable-options --output-format json # 成功
```

### 本 workspace 的 toolchain 策略

[`rust-toolchain.toml`](./rust-toolchain.toml) 钉 **Stable**（fmt / clippy / test / MSRV 日常开发）：

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

部分 CI job 依赖 **Nightly 工具链能力**（不是换 Edition 写法）：

| Job | 原因 | 覆盖方式 |
|-----|------|----------|
| `miri` | Miri 组件仅随 Nightly 分发 | `RUSTUP_TOOLCHAIN=nightly` |
| `public-api` | `cargo public-api` 需 rustdoc JSON | 同上 + `dtolnay/rust-toolchain@nightly` |

CI 里用环境变量 **覆盖** 本目录 `rust-toolchain.toml`，避免在 Stable 下误跑 miri / public-api。本地同理：

```bash
# 日常（尊重 rust-toolchain.toml → stable）
cargo test --workspace

# Miri / public-api（显式 nightly）
$env:RUSTUP_TOOLCHAIN = "nightly"   # PowerShell
cargo miri test -p item-16-miri

cd item-24-re-export/dep-lib
cargo public-api
```

## Item 25：`[workspace.dependencies]`

成员 crate 通过 `{ workspace = true }` 引用统一版本，例如 `item-04-error-types` 的 `anyhow` / `thiserror`。

### Dependabot + `cargo deny`

- [`.github/dependabot.yml`](../../../.github/dependabot.yml) — 每周更新 `ER/ER-demos` 等 lock
- 根目录 [`deny.toml`](../../../deny.toml) — advisories / licenses / bans
- CI `cargo-deny` job：`cargo deny check all`

本地：

```bash
cargo install cargo-deny
cargo deny check all
```

## Item 21：`cargo-semver-checks`

未发布 crate 用 **git baseline** 对比 PR 与 base 分支的 public API：

```bash
cd ER/ER-demos/item-24-re-export
cargo install cargo-semver-checks --locked
cargo semver-checks --package dep-lib --baseline-rev main
```

CI：`semver-checks` job（仅 `pull_request`），见 [item-24-re-export/README.md](./item-24-re-export/README.md)。

## Item 29：`clippy.toml`

本目录 [`clippy.toml`](./clippy.toml) 对 workspace 内 crate 生效。
