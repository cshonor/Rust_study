# 2. Workspaces（工作区）

> [← 章索引](./README.md)

巨石单 crate → 改一行触发大范围重编；**workspace** 拆分成员 crate。

## 声明

根 **`Cargo.toml`**：

```toml
[workspace]
members = ["crates/foo", "crates/bar"]
resolver = "2"
```

## 共享

- **`Cargo.lock`**（根一致解析）
- **`target/`** 与增量产物

## 工作流

根目录 **`cargo test` / `cargo check`** 覆盖成员；变更时往往只重建**受影响成员及其依赖方**。

Book → [14.3 Cargo 工作空间](../../00-Book/14-cargo-crates/14.3-Cargo工作空间.md) · demo → [14.3-workspace-demo](../../00-Book/14-cargo-crates/14.3-workspace-demo/)
