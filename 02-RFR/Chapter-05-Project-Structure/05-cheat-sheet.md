# 3.2 · Build Configuration · 速记

← [05 构建配置](./05-build-configuration.md) · [03 工作区](./03-workspaces.md) · [章索引](./README.md)

---

## [patch]

仅**根**生效 · 临时修上游 · 发布前删 · path/git 互斥

```toml
[patch.crates-io]
serde = { path = "../fix" }
```

## profile 四参

| 参 | 要点 |
|----|------|
| `opt-level` | 0 调试 · 3 快 · s/z 小 |
| `codegen-units` | 大=编译快 · 1=性能峰 |
| `lto` | false/thin/fat |
| `panic` | unwind 默认 · abort 瘦身 |

## release 模板

```toml
opt-level = 3
codegen-units = 1
lto = true
panic = "abort"
strip = "symbols"
```

## Workspace

profile/patch **仅根** · `[profile.ci] inherits = "release"`

## 自测

- [ ] 子 crate 写 `[patch]` 为何无效？  
- [ ] `panic=abort` 对 `cargo test` 有何影响？  
- [ ] `codegen-units=1` 换什么？
