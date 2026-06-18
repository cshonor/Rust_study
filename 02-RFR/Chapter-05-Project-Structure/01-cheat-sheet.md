# 1.1 · Cargo Feature · 速记

← [01 定义与包含 Feature](./01-defining-including-features.md) · [章索引](./README.md)

---

## 两种写法

```toml
# 使用者：依赖上直接开
serde = { version = "1", features = ["derive"] }
# 库作者：自有 feature 中转
serde-full = ["dep:serde", "serde/derive"]
```

## 两种 Feature 别混

第三方 `serde/json` · 自己的 `[features]` 段

## Additive 原则（仅库作者设计自有 feature）

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

- [ ] `dependencies` 里 `features = []` 和自有 `[features]` 差在哪？  
- [ ] 互斥禁令约束使用者还是库作者？  
- [ ] `serde-full = ["dep:serde", "serde/derive"]` 各段含义？
