# 5.4 · 未发布版本 · 速记

← [10 Unreleased Versions](./10-unreleased-versions.md) · [09 Changelog](./09-changelogs.md) · [章索引](./README.md)

---

## [Unreleased]

顶部累积 · 发版 → `[X.Y.Z] - 日期` · 清空重来

## 预发布

`alpha` < `beta` < `rc` < 正式

## 依赖预发布

必须写 `2.0.0-rc.1` — `"2.0.0"` 不匹配 rc

## 发版三步

1. bump `Cargo.toml` version  
2. 迁移 CHANGELOG  
3. merge → `git tag vX.Y.Z`

## 禁

先 tag 后改 version / CHANGELOG

## 自测

- [ ] 为何 `"2.0.0"` 拉不到 `2.0.0-rc.1`？  
- [ ] 发版 PR 应同时改哪两个文件？  
- [ ] `[Unreleased]` 发版后如何处理？
