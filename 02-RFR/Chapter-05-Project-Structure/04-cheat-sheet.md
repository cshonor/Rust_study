# 3.1 · Crate Metadata · 速记

← [04 Crate 元数据](./04-crate-metadata.md) · [章索引](./README.md)

---

## 本质

发布 / 展示 / 检索 / 合规 — **多数不影响编译**

## vs 构建配置

| Metadata `[package]` | 构建 `[profile]` |
|---------------------|------------------|
| crates.io 展示 | 优化 / 编译开关 |

**例外**：`edition` 在 `[package]` 但**影响编译**

## 身份 & 合规

`name` · `version` · `authors` · `license` · `repository`

## 发现

`description` · `keywords` (≤5) · `categories` · `readme`

## Workspace

根 `[workspace.package]` + 子包 `version.workspace = true` 等

## 场景

私有：name+version+edition · 开源：补 license/repo/keywords

## 发布前

```bash
cargo package --list
```

## 自测

- [ ] metadata 和 `[profile]` 谁影响二进制？  
- [ ] `edition` 算纯展示吗？  
- [ ] 工作区如何统一 authors/license？
