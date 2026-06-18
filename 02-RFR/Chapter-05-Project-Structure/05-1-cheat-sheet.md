# Workspace + Patch + Profile · 汇总速记

← [05.1 完整汇总](./05-1-workspace-patch-profile-汇总.md) · [05 构建配置](./05-build-configuration.md)

---

## 一句话

Patch/Profile **仅根** · inherits **根上新 profile** · 单包微调 **`profile.xxx.package.包名`**

## [patch]

| 块 | 替换 |
|----|------|
| `[patch.crates-io]` | registry 依赖 |
| `[patch."git-url"]` | 同 URL 的 git 依赖 |

不 publish · 子 crate 无效 · 多块分来源

## [profile]

dev / release / test / bench · 自定义 `ci` 等 — **仅根**

## inherits

```toml
[profile.ci]
inherits = "release"
lto = false
```

根上定义 · 复制父 profile 再局部覆盖

## 单包微调（不是子 crate profile）

```toml
[profile.release.package.my-lib]
opt-level = 2
```

## 误区

❌ 子 crate `[profile.release] inherits = …` → **被忽略**  
✅ 根 `[profile.release.package.my-lib]`

## 自测

- [ ] patch 写子 crate 会生效吗？  
- [ ] 只让 my-lib release 用 opt-level 2 怎么写？  
- [ ] `inherits` 能写在 member Cargo.toml 吗？
