# 2 · Workspaces · 速记

← [03 Workspaces](./03-workspaces.md) · [章索引](./README.md)

---

## 痛点 → 价值

巨石全量重编 → 多 member · 增量 · 边界 · monorepo

## 根 TOML

```toml
[workspace]
members = ["crates/*", "cli"]
resolver = "2"
```

虚拟 = 无 `[package]` · 带包 = 根也是 crate

## 两大共享

`Cargo.lock` 唯一 · `target/` 统一

## 仅根生效

`profile` · `patch` · `[workspace.dependencies]` · `[workspace.package]`

## 命令

`--workspace` 全局 · `-p foo` 单包

## resolver 2

普通 / build / proc-macro feature 隔离 · 平台依赖隔离 · 虚拟 ws 须手写

## 子包依赖

```toml
tokio = { workspace = true }
foo = { path = "../foo", version = "0.1" }
```

## 自测

- [ ] 为何子 crate 改 profile 无效？  
- [ ] 改 `foo` 后哪些包会重编？  
- [ ] 虚拟工作区为何不自动 resolver 2？
