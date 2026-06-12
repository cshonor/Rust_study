# 1.1 Defining and Including Features（定义与包含 Feature）

> 所属：**Features** · [← 章索引](./README.md)

**Feature** = 传给依赖解析与编译器的**构建开关**。

## 附加性 (Additive) 原则

特性应**增加**能力，避免用 feature **互斥**地：

- 删除模块、替换公开类型、改变函数签名。

**原因**：Cargo 对同一依赖取 feature **并集**后只编一份；互斥 feature 合并后可能无法编译。

## 可选依赖

```toml
[dependencies]
serde = { version = "1", optional = true }

[features]
default = ["serde"]
serde = ["dep:serde"]
```

- **`optional = true`** — 默认不拉依赖，缩短编译。
- **`default-features = false`** — 下游只开所需子集。

ER → [Item 26 feature creep](../../01-ER/Chapter-04-Dependencies/Item-26-feature-creep/README.md)
