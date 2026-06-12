# 5.3 Changelogs（变更日志）

> 所属：**Versioning** · [← 章索引](./README.md)

不要只靠 Git log — 维护人类可读的 **CHANGELOG**（如 [Keep a Changelog](https://keepachangelog.com/)）。

## 应突出

- **Breaking changes**
- **MSRV** 变化
- **Feature 默认值**变化
- 安全修复

## 与 SemVer 对齐

- Major — 破坏性 API 变更
- Minor — 向后兼容功能 / MSRV bump（生态惯例）
- Patch — 修复

ER → [Item 21](../../01-ER/Chapter-04-Dependencies/Item-21-semver/README.md)
