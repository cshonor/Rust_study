# 3.2 · Build Configuration · 速记

← [05 构建配置](./05-build-configuration.md) · [03 工作区](./03-workspaces.md) · [章索引](./README.md)

---

## 三模块

| 模块 | 要点 |
|------|------|
| `[patch]` | 临时换依赖源 · 仅根 · 不 publish |
| `[profile.*]` | 编译优化 dev/release/test/bench |
| Workspace | profile/patch **仅根** · `inherits` |

## [patch]

```toml
[patch.crates-io]
serde = { path = "./local-fix-serde" }
```

不随 publish · 子 crate 写无效 · 修完删

## profile 四参

`opt-level` · `codegen-units` · `lto` · `panic` (+ `strip`)

| 内置 | 命令 |
|------|------|
| dev | `cargo build` |
| release | `--release` |
| test/bench | `cargo test` / `bench` |

## inherits（根目录）

```toml
[profile.ci]
inherits = "release"
lto = false
```

## vs Metadata

metadata = 展示 · profile/patch = 编译/解析

## 自测

- [ ] patch 会打进 crates.io 发布包吗？  
- [ ] 子 crate 写 `[profile.release]` 生效吗？  
- [ ] `inherits = "release"` 写在哪一层 TOML？
