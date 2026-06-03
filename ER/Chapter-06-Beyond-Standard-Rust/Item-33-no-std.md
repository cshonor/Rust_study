# Item 33: Consider making library code no_std compatible

> **Effective Rust** · [Chapter 6 — Beyond Standard Rust](../ER-本书目录.md)  
> **中文**：考虑让库代码兼容 no_std  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-33-no-std](../ER-demos/item-33-no-std/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `core` 与所有权基础 | [4.1 所有权](../../Book/04-ownership/4.1-什么是所有权.md) |
| `Mutex` / 并发（no_std 缺） | [16.3 共享状态](../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| `HashMap`（no_std 常缺） | [8.3 HashMap](../../Book/08-collections/8.3-hashmap.md) |
| Features 加法原则 | [Item 26](../Chapter-04-Dependencies/Item-26-feature-creep.md) |
| 零拷贝 vs 易用（嵌入式硬约束） | [Item 20](../Chapter-03-Concepts/Item-20-avoid-over-optimize.md) |
| CI 交叉编译 | [Item 32](../Chapter-05-Tooling/Item-32-ci.md) |

---

## 1. 核心知识点与关键定义

### 三层库

| 层 | 说明 |
|----|------|
| **`std`** | 默认；OS、I/O、线程、完整集合 |
| **`core`** | 始终可用；**无堆分配**；`Option`、`Result`、`Iterator`… |
| **`alloc`** | 需 `extern crate alloc;`；`Vec`、`Box`、`String`、`Rc`/`Arc` |

### `no_std`

```rust
#![no_std]
```

- 不链接 **`std`**；用于引导程序、固件、裸机。
- 许多 `std::` 基础类型实为 **`core::` 重导出**。

---

## 2. 逻辑脉络

```text
库兼容 no_std → 嵌入式 / 内核用户也能用
         ↓
常只需 std:: → core:: 替换
         ↓
无 OS：无 HashMap（随机种子）、无 Mutex（OS 原语）
         ↓
用 BTreeMap、spin 等替代
         ↓
Feature "std" / "alloc" 加法门控（Item 26）
         ↓
CI --target thumbv6m-none-eabi 守卫（Item 32）
```

---

## 3. 重点结论与实用要点

### CI 是唯一可靠守卫

- 无意间依赖带 `std` 的 crate → **编译器不专门警告**。
- CI：`cargo build --no-default-features --target thumbv6m-none-eabi`（或项目实际 target）。

### Feature 隔离（加法！）

```toml
[features]
default = ["std"]
std = ["alloc"]
alloc = []
```

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
```

- ❌ **`no_std` feature 删代码** — Item 26：unification 并集会炸。
- ✅ **`std` feature 加能力**。

### OOM 与 fallible allocation

- `alloc` 默认：**分配失败 → panic**（`Vec::push`）。
- 嵌入式：用 **`try_reserve`**、**`Box::try_new`** 等返回 `Result`。

---

## 4. 案例与代码要点

### `no_std` + 可选 `alloc`

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
```

### Fallible 构建 `Vec`

```rust
fn try_build_a_vec() -> Result<Vec<u8>, String> {
    let mut v = Vec::new();
    let required_size = 4;
    v.try_reserve(required_size)
        .map_err(|_| format!("Failed to allocate {} items!", required_size))?;
    v.push(1);
    Ok(v)
}
```

### `no_std` 下常见替代

| `std` 有 | `no_std` 替代 |
|----------|----------------|
| `HashMap` / `HashSet` | **`BTreeMap` / `BTreeSet`** |
| `Mutex` / `RwLock` | **`spin`** 等自旋锁 crate |
| `std::thread` | 平台特定 / 无 |

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **`no_std` / `no_alloc` feature 名** | 违反加法；用 **`std` / `alloc`** 正向开启 |
| **文档路径 `src/core/`** | 该类型 **`no_std` 可用** |
| **隐式 `push`/`collect`** | OOM panic；受限环境改 fallible API |
| **只本地测 default features** | CI 必须 **no-default + bare-metal target** |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 33](../ER-拓展索引.md#item-33)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 栈 | **core**（无堆）→ **alloc**（堆）→ **std**（OS） |
| 声明 | `#![cfg_attr(not(feature="std"), no_std)]` |
| Feature | **`std` 加法**，别 `no_std` 减法 |
| 守卫 | **CI 交叉编译** bare-metal |
| OOM | **`try_reserve`** / `try_new` |
| 缺啥 | HashMap→BTree；Mutex→spin |
