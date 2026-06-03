# Item 23: Avoid wildcard imports

> **Effective Rust** · [Chapter 4 — Dependencies](../ER-本书目录.md)  
> **中文**：避免通配符导入  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo / 代码练习

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `use`、glob `*` | [7.4 使用 use 引入名称](../../Book/07-packages-modules/7.4-使用use关键字将名称引入作用域.md) |
| 测试里 `use super::*` | [11.3 测试的组织结构](../../Book/11-testing/11.3-测试的组织结构.md) |
| Minor 加 API 仍「兼容」 | [Item 21](./Item-21-semver.md) |
| 可见性 / 重导出 | [Item 22](./Item-22-minimize-visibility.md)、[Item 24](./Item-24-re-export-api-types.md)（待补） |
| Clippy 警告 | [Item 29](../Chapter-05-Tooling/Item-29-clippy.md)（待补） |

---

## 1. 核心知识点与关键定义

### 通配符导入（Wildcard / Glob）

```rust
use somecrate::module::*;
```

- 将目标模块**全部公开符号**一次性拉入当前作用域。

### 命名空间污染

- 外部 crate 演进时，未知符号静默进入本地命名空间 → **冲突**、**歧义**、难维护。

---

## 2. 逻辑脉络

```text
Item 21：Minor 加公开 API = Semver 兼容
         ↓
use external::* + cargo update → 新符号静默出现
         ↓
普通类型同名：本地定义优先 → 往往无害
Trait 方法同名：方法解析歧义 → 编译失败（E0034）
         ↓
对策：第三方 crate 显式 use；例外见 §3
```

---

## 3. 重点结论与实用要点

### 默认：第三方 crate **不用** `*`

- 明确列出需要的类型 / trait / 函数。

### 合法例外

| 场景 | 示例 | 为何可接受 |
|------|------|------------|
| **测试** | `use super::*;` | 你控制模块与测试（Book 11.3） |
| **crate 内重导出** | `pub use thing::*;` | 同一 crate，你控制符号集（Item 22/24） |
| **库作者 curated prelude** | `use thing::prelude::*;` | 作者维护的小集合，生态惯例 |

### 硬要用 `*` 时的兜底

- `Cargo.toml` **精确锁定**版本（`=1.2.3`，见 Item 21）—— 仍不推荐，只是减缓 Minor 炸弹。

---

## 4. 案例与代码要点

### 本地名称掩蔽（通常无害）

```rust
use bytes::*;

struct Bytes(Vec<u8>); // 本地 Bytes 优先，不与 bytes::Bytes 冲突
```

普通类型冲突时，**本地定义优先**于 glob 导入。

### Trait 方法冲突（编译失败）

- 依赖 Minor 新增 trait `BytesLeft`，为 `&[u8]` 提供 `remaining()`
- 你已 glob 导入 `bytes::Buf`，也有 `remaining()`
- `v.remaining()` → **`E0034: multiple applicable items in scope`**

修复：显式导入 + **完全限定语法**：

```rust
BytesLeft::remaining(&v)
// 或 Buf::remaining(&v)
```

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **忽略 Clippy** | glob import 有相关 lint（Item 29）；图省事 = 埋定时炸弹 |
| **以为本地优先万能** | 只救**类型名**；**trait 方法**仍歧义 |
| **`cargo update` 无感 break** | 依赖范围未锁 Major，Minor 加 trait 即可炸 build |

---

## 6. 后续拓展（待补）

- [ ] 设计 crate 自有 **`prelude`**：与 Item 22 可见性 + Item 24 重导出配合
- [ ] `std::prelude` / 常用库 prelude 对比

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 规则 | **第三方不用 `*`** |
| 例外 | 测试、`pub use`、curated prelude |
| Semver | Minor 加符号 + glob = 静默风险 |
| 类型冲突 | 本地优先，常无害 |
| trait 冲突 | 方法歧义 → **E0034** |
| 兜底 | `=x.y.z` 锁版本（下策） |
