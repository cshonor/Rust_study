# 2 · Workspaces · 速记

← [03 Workspaces](./03-workspaces.md) · [章索引](./README.md)

---

## 核心结论

根 TOML **不管**子包 version / 业务依赖注入 · 只管 **members** + 全局 **Cargo.lock**

子包提版本要求 → lock 统一落地 → 同一第三方 crate 不会多版本并存

## members

路径列表 → 指向**含 Cargo.toml 的文件夹**（不是 TOML 文件路径）

`crates/*` 通配直接子目录 · 不在 members = 不共用 lock / 不能 `workspace = true`

根无 `[package]` = 不是包 · 一个 Package 只能属一个 Workspace

## 根能干 / 不能干

✅ members · lock · 批量 cargo · `workspace.dependencies` / `profile` / `patch`  
❌ 不给所有子包自动注入 `[dependencies]` · 不替子包定 `[package].version`

## 子包自治

各自 `version` · `[dependencies]` · path 引用兄弟包

```toml
core-utils = { path = "../../crates/core-utils", version = "0.1.0" }
```

## 依赖版本两种模式

| 经典 | 各子包自己写版本，lock 收敛 |
| 现代 | 根 `[workspace.dependencies]` + `tokio = { workspace = true }` |

## 两大共享

`Cargo.lock` 唯一 · `target/` 统一

## 命令

`--workspace` 全局 · `-p foo` 单包 · `default-members` 无 -p 默认谁

## resolver 2

虚拟 ws 须手写 · feature / 平台依赖隔离

## 自测

- [ ] `members` 填的是文件夹还是 `Cargo.toml` 路径？  
- [ ] 有 Cargo.toml 但没进 members 的包能 `workspace = true` 吗？  
- [ ] 子包 A/B 对 tokio 约束不同，最终用几个版本？
