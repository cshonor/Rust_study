# 3.1 Crate Metadata（Crate 元数据）

> 所属：**Project Configuration** · [← 章索引](./README.md)

`Cargo.toml` 中影响**发布与发现**的字段：

| 字段 | 作用 |
|------|------|
| `name` / `version` | 包标识与 SemVer |
| `description` / `documentation` | crates.io 与 docs.rs |
| `license` / `repository` | 合规与溯源 |
| `keywords` / `categories` | 生态检索 |
| `readme` | crates.io 首页 |
| `exclude` / `include` | 控制发布 tarball 内容 |

## 实践

- 发布前 `cargo package --list` 检查打进 crate 的文件。
- 敏感路径用 `.cargo_vcsignore` / `exclude` 排除。

Book → [14.1 发布到 crates.io](../../00-Book/14-cargo-crates/14.1-发布到-crates-io.md) · ER → [Item 27 文档](../../01-ER/Chapter-05-Tooling/Item-27-document-public-api/README.md)
