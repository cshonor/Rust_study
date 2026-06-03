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
