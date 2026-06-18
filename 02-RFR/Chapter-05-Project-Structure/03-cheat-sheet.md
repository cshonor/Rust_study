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

## 版本合并 vs 冲突

同 crate 全 ws **只锁一个版本**

| 情况 | 结果 |
|------|------|
| `"1.0"` + `"1.5"` | 交集 `≥1.5,<2` → 全局 1.5+ 最新 |
| `"1"` + `"2"` | 无交集 → **构建失败** |

SemVer 前提 · 跨大版本不能共存 → 升级代码 / 拆 ws / patch（少用）

## 依赖版本两种模式

| 经典 | 各子包自己写版本，lock 收敛 |
| 现代 | 根 `third = "1.5.10"` + `third = { workspace = true }` |

## 两大共享

`Cargo.lock` 唯一 · `target/` 统一 · 详 [03.1 lock](./03-1-cargo-lock.md)

## 命令

`--workspace` 全局 · `-p foo` 单包 · `default-members` 无 -p 默认谁

## resolver 2

虚拟 ws 须手写 · feature / 平台依赖隔离

## 自测

- [ ] crate1=`"1.0"` crate2=`"1.5"` 最终 lock 里几个 `third` 版本？  
- [ ] crate1=`"1"` crate2=`"2"` 能编过吗？为什么？  
- [ ] 必须 1.x 和 2.x 并存，Workspace 标准方案够吗？
