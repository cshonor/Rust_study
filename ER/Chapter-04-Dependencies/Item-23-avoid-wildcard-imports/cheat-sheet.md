# Item 23 · 记忆卡片

← [Item 23 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 规则 | **第三方不用 `*`** |
| 例外 | 测试、`pub use`、curated prelude |
| Semver | Minor 加符号 + glob = 静默风险 |
| 类型冲突 | 本地优先，常无害 |
| trait 冲突 | 方法歧义 → **E0034** |
| 兜底 | `=x.y.z` 锁版本（下策） |
