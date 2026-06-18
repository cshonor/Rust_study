# 1.2 · crate 内使用 Feature · 速记

← [02 Using Features](./02-using-features-in-crate.md) · [01 定义](./01-defining-including-features.md) · [章索引](./README.md)

---

## 底层

**加法模型 · Feature Unification · 全局合并 · 无法局部关闭**

## 代码模板

```rust
#[cfg(feature = "serde")]
mod serde_impl { /* … */ }

#[cfg_attr(feature = "serde", derive(Serialize))]
```

收拢 mod · 全 guarded · `all/any/not`

## TOML 模板

```toml
serde = { optional = true }
[features]
serde = ["dep:serde"]
full = ["serde", "async"]
std = []
```

## CI 必跑

```bash
cargo check --no-default-features
cargo check --all-features
```

## 互斥后端 → 别用 feature

1. 拆 crate  
2. 运行时选  
3. `compile_error!` 兜底  

## 背诵六点

加法合并 · cfg mod · dep:xxx · 双 CI · 禁互斥后端 · 文档标 feature

## 自测

- [ ] 为何下游无法「只关 serde 不关 tokio 的 serde feature」？  
- [ ] `--all-features` 测什么失败场景？  
- [ ] `std = []` 空 feature 有什么用？
