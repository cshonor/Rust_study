# 5.3 · Changelog · 速记

← [09 Changelogs](./09-changelogs.md) · [07 MSRV](./07-msrv.md) · [章索引](./README.md)

---

## 原则

独立 **CHANGELOG.md** · Keep a Changelog · 不靠 Git log

## 五类必写

Breaking · MSRV · Feature 默认 · 安全 · 常规

## SemVer

| 版本 | 内容 |
|------|------|
| Major | Breaking |
| Minor | 新功能 / **抬 MSRV** |
| Patch | 修复 / 安全 — **禁 MSRV** |

## 结构

倒序 · 分区块 · `[Unreleased]` 待发 · 迁移示例

## 避坑

用户视角 · MSRV 不进 patch · 安全高亮 · README 同步

## 自测

- [ ] MSRV 应写在 Minor 还是 Patch？  
- [ ] `[Unreleased]` 何时改成版本号？  
- [ ] default feature 变更为何要写 CHANGELOG？
