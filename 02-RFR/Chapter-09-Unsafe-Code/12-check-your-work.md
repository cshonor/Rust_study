# 4.3 Check Your Work（验证）

> 所属：**Coping with Fear** · [← 章索引](./README.md)

## Miri

```bash
cargo miri test
```

捕捉未初始化读、悬垂、别名违规等（以 Miri 支持范围为准）。

→ [ER Item 16 demo](../../01-ER/Chapter-03-Concepts/Item-16-avoid-unsafe/demo/)

## 其他

- **单元 / 集成测试** — 边界输入、panic 路径
- **FFI** — valgrind / sanitizer（平台相关）
- **Review** — 第二人审 unsafe 块

Book demo → [19.1 unsafe demo](../../00-Book/19-advanced-features/19.1-unsafe-rust-demo/)
