# 5.4 Unreleased Versions（未发布版本）

> 所属：**Versioning** · [← 章索引](./README.md)

← [09 Changelog](./09-changelogs.md) · 下一节 [11 小结](./11-summary.md)

前置 → [09 Changelog §四 Unreleased](./09-changelogs.md#四keep-a-changelog-书写规范)

## `Unreleased` 小节

CHANGELOG 顶部保留 **`[Unreleased]`**，累积即将发布的变更，发版时改为版本号 + 日期。

```markdown
## [Unreleased]
### Added
- ...

## [1.2.0] - 2026-01-01
```

## 预发布 SemVer

- **`1.0.0-alpha.1`**、**`-beta`**、**`-rc`** — 语义见 SemVer 预发布规则。
- 依赖预发布 crate 时需显式接受预发布标识。

## 与 git 协作

- 发版 PR： bump `version` + 移动 CHANGELOG + tag。
- 避免「打了 tag 却忘了改 Cargo.toml」。
