# 3.1 · Crate Metadata · 速记

← [04 Crate 元数据](./04-crate-metadata.md) · [章索引](./README.md)

---

## 本质

发布 / 展示 / 检索 / 合规 — **不影响编译**

## 身份 & 合规

`name` · `version` (SemVer) · `license` · `repository`

## 发现

`description` · `keywords` (≤5) · `categories` · `readme` · `documentation`

## 打包控制

`include` 白名单 · `exclude` 黑名单 · `.cargo_vcsignore`

## 发布前

```bash
cargo package --list
```

## 自测

- [ ] `exclude` 和 `.gitignore` 差在哪？  
- [ ] 为何不填 `license` 无法 publish？  
- [ ] `keywords` 最多几个？
