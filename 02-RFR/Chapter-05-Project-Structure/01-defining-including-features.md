# 1.1 Defining and Including Features（定义与包含 Feature）

> 所属：**Features** · [← 章索引](./README.md)

← [章索引](./README.md) · 下一节 [02 crate 内使用 Feature](./02-using-features-in-crate.md)

> ER → [Item 26 · Feature Creep](../../01-ER/Chapter-04-Dependencies/Item-26-feature-creep/README.md)

---

## 一、核心定义

**Feature** = 传给 Cargo **依赖解析器** + Rust **编译器**的**构建开关**，用于：

1. 控制是否编译**可选依赖**  
2. 控制代码内 **`#[cfg(feature = "xxx")]`** 条件编译分支  
3. 对外提供功能子集 — 用户按需开启，减少编译体积  

---

## 二、附加性 Additive 原则（硬性规范）

所有 Feature 只能**叠加新增功能**，**不能**做破坏性修改。

### 禁止行为

1. 删掉原有模块、结构体、函数  
2. 替换已公开的类型、修改函数入参 / 返回签名  
3. 两个 Feature **互斥**（开 A 就不能开 B）  

### 底层原因

Cargo 对同一库的所有 Feature 做**并集合并**，只编译**一份**代码。  
互斥 Feature 或修改同一段代码 → 合并后冲突 → 编译失败。

> **设计思路**：任意多个 Feature 同时开启，代码必须**合法可编译**。

→ 详 [02 crate 内使用](./02-using-features-in-crate.md) · demo：[Item 26 demo](../../01-ER/Chapter-04-Dependencies/Item-26-feature-creep/demo/)

---

## 三、可选依赖写法

### 1. 声明可选依赖

```toml
[dependencies]
# 默认不自动安装、编译
serde = { version = "1.0", optional = true }
```

`optional = true` — 标记可选，默认不拉取，缩短基础编译时间。

### 2. 把依赖绑定成 Feature

```toml
[features]
# 开启 serde 特性 = 启用可选依赖 serde
serde = ["dep:serde"]
```

`dep:xxx` — Cargo 专用语法：「该 feature 启用名为 `xxx` 的可选依赖」。

### 3. `default` 默认特性

```toml
[features]
default = ["serde"]
serde = ["dep:serde"]
```

下游引用时**默认**开启 `serde`。轻量化编译：

```toml
# 下游 Cargo.toml
mylib = { version = "0.1", default-features = false, features = [] }
```

| 选项 | 含义 |
|------|------|
| `default-features = false` | 关闭全部默认特性 |
| `features = ["serde"]` | 只手动开启需要的子集 |

---

## 四、与代码侧配合（预览）

```toml
# Cargo.toml（库作者）
[dependencies]
serde = { version = "1", optional = true }

[features]
default = ["serde"]
serde = ["dep:serde"]
```

```rust
// lib.rs
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Record {
    pub data: Vec<u8>,
}

#[cfg(feature = "serde")]
impl Record {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
```

→ 详 [02 crate 内使用](./02-using-features-in-crate.md) · demo：[Item 26 demo](../../01-ER/Chapter-04-Dependencies/Item-26-feature-creep/demo/)

---

## 五、ER Item 26：Feature Creep（特性泛滥）

**忠告**：不要无限堆砌细碎小 Feature。

| 问题 | 后果 |
|------|------|
| Feature 过多 | 下游难选、配置复杂 |
| 依赖树膨胀 | 编译时间变长 |
| 大量 `cfg(feature)` | 可读性下降 |

**建议**：功能相近的能力**合并**成一个大 Feature，避免碎片化开关。

---

## 六、核心速记

1. Feature = 构建开关 — **只能加功能**，不能改删现有 API。  
2. `optional = true` + `dep:xxx` 绑定可选依赖。  
3. `default` = 开箱即用集合；下游 `default-features = false` 精简。  
4. **禁止互斥** — Cargo 会合并所有开启的 Feature。  

→ 速记：[01-cheat-sheet.md](./01-cheat-sheet.md) · 下一节：[02 crate 内使用](./02-using-features-in-crate.md)
