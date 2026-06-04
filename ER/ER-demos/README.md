# Effective Rust — Demo 索引

拓展全文：[ER-拓展索引.md](../ER-拓展索引.md) · Workspace 配置：[WORKSPACE.md](./WORKSPACE.md) · Rust 工具链概念：[README § 工具链](../../README.md#rust-工具链stablenightly--edition)

## 运行

```bash
cd ER/ER-demos
cargo test --workspace
cargo run -p item-05-06-newtype
cd item-24-re-export && cargo run -p consumer-newtype
cd item-35-sys-workspace && cargo test --workspace
```

## Item → Demo

| Item | ER demo |
|------|---------|
| 3 | [item-03-option-result](./item-03-option-result/) |
| 4 | [item-04-error-types](./item-04-error-types/) · [item-04-core-error](./item-04-core-error/) |
| 5–6 | [item-05-06-newtype](./item-05-06-newtype/)（Deref / derive_more / AsRef·Into / 过度 Deref） |
| 16 | [item-16-miri](./item-16-miri/) + CI `miri` job |
| 15 | [item-15-borrow-checker](./item-15-borrow-checker/) |
| 18 | [item-18-dont-panic](./item-18-dont-panic/) |
| 20 | [item-20-tlv](./item-20-tlv/) |
| 21 | [WORKSPACE.md](./WORKSPACE.md) MSRV + CI `msrv` / `semver-checks` |
| 22 | [item-22-visibility](./item-22-visibility/) |
| 24 | [item-24-re-export](./item-24-re-export/) |
| 25 | [Cargo.toml](./Cargo.toml) + [deny.toml](../../deny.toml) + Dependabot |
| 26 | [item-26-feature-creep](./item-26-feature-creep/) |
| 29 | [clippy.toml](./clippy.toml) |
| 30 | [item-30-black-box](./item-30-black-box/) + CI `matrix-demo` |
| 32 | [er-study-ci.yml](../../.github/workflows/er-study-ci.yml) |
| 33 | [item-33-no-std](./item-33-no-std/) |
| 34 | [item-34-ffi-box](./item-34-ffi-box/) |
| 35 | [item-35-bindgen](./item-35-bindgen/) · [item-35-sys-workspace](./item-35-sys-workspace/) |

其余 Item 见拓展索引中的 Book demo 链接。
