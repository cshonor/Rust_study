# 5.1 Minimum Supported Rust Version（MSRV）

> 所属：**Versioning** · [← 章索引](./README.md)

## 生态约定

提升 **MSRV** 往往按 **minor** 处理（如 `2.6 → 2.7`），让旧工具链用户可把依赖**上限锁在 `< 2.7`**。

## 实践

- `README` / `Cargo.toml` **`rust-version`** 字段声明 MSRV。
- CI 用该版本跑 `cargo test`。
- 需要新语法时在 CHANGELOG 写明 MSRV bump。

ER → [Item 21 SemVer](../../01-ER/Chapter-04-Dependencies/Item-21-semver/README.md)
