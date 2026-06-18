# 1.1 · Cargo Feature · 速记

← [01 定义与包含 Feature](./01-defining-including-features.md) · [章索引](./README.md)

---

## Feature 三件事

可选依赖 · `#[cfg(feature)]` · 功能子集减体积

## Additive 原则

只加不删 · 不改公开 API · **禁止互斥**（Cargo 取并集）

## TOML 三板斧

```toml
serde = { optional = true }
[features]
default = ["serde"]
serde = ["dep:serde"]
```

## 下游精简

`default-features = false` + 手动 `features = [...]`

## Feature Creep

少而粗 · 合并相近能力 · 少 `cfg` 污染

## 自测

- [ ] 为何不能设计「开 A 必关 B」的互斥 Feature？  
- [ ] `dep:serde` 和直接写 `serde` 在 features 里差在哪？  
- [ ] `--no-default-features` 测什么？
