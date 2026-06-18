# 1.2 Using Features in Your Crate（crate 内使用 Feature）

> 所属：**Features** · [← 章索引](./README.md)

← [01 定义与包含 Feature](./01-defining-including-features.md) · 下一节 [03 工作区](./03-workspaces.md)

前置 → [01 Additive 原则](./01-defining-including-features.md) · [06 条件编译](./06-conditional-compilation.md)

## 代码侧

```rust
#[cfg(feature = "serde")]
mod serde_impl { ... }
```

## 依赖侧

在 `Cargo.toml` 为可选依赖绑定 feature；**不要**在多个 feature 里重复启用冲突语义。

## 文档与 CI

- README 说明各 feature 含义与默认组合。
- CI 矩阵：至少跑 **`--no-default-features`** 与 **`--all-features`**（若合理）各一条。

## 反模式

- 用 feature 做「互斥后端 A / B」且未保证并集安全 → 拆 crate 或统一抽象层。

Demo → [Item 26 demo](../../01-ER/Chapter-04-Dependencies/Item-26-feature-creep/demo/)
