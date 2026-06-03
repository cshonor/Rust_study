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

## Item 29：`clippy.toml`

本目录 [`clippy.toml`](./clippy.toml) 对 workspace 内 crate 生效。
