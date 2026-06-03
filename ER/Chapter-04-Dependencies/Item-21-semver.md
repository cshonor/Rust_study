# Item 21: Understand what semantic versioning promises

> **Effective Rust** · [Chapter 4 — Dependencies](../ER-本书目录.md)  
> **中文**：理解语义化版本控制的承诺  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| Cargo、`Cargo.toml` | [1.3 Hello Cargo](../../Book/01-getting-started/1.3-Hello-Cargo.md) |
| 发布、版本不可变、yank | [14.2 发布到 Crates.io](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| 工作空间与 `Cargo.lock` | [14.3 Cargo 工作空间](../../Book/14-cargo-crates/14.3-Cargo工作空间.md) |
| trait 对象安全 | [19.2 高级 trait](../../Book/19-advanced-features/19.2-高级trait.md) |
| 缩小 API 面 | [Item 22](./Item-22-minimize-visibility.md)（待补） |
| 通配符导入冲突 | [Item 23](./Item-23-avoid-wildcard-imports.md)（待补） |
| CI / Dependabot | [Item 32](../Chapter-05-Tooling/Item-32-ci.md)（待补） |

---

## 1. 核心知识点与关键定义

### Semver（Semantic Versioning）

Cargo 用 Semver 解析依赖。格式 **`MAJOR.MINOR.PATCH`**：

| 段 | 何时递增 |
|----|----------|
| **MAJOR** | **不兼容** API 变更 |
| **MINOR** | **向后兼容**地加功能 |
| **PATCH** | **向后兼容** Bug 修复 |

### 发布不可变

- 某版本一旦发布 → **内容不可改**；任何变动必须是**新版本**（Book 14.2；yank 只阻止新解析，不删源码）。

### `0.y.z` 特殊规则（left-shifting）

- `0.x` = 初始开发，API 随时可能 break。
- Cargo：**最左侧非零位**变化即视为不兼容：
  - `0.2.3` → `0.3.0` ✓ break
  - `0.0.4` → `0.0.5` ✓ break

---

## 2. 逻辑脉络

```text
Semver 理论：三条规则，清晰明了
         ↓
实践：何谓「向后兼容」充满灰色
         ↓
Hyrum 定律：用户会依赖未文档化的行为
         ↓
作者：少暴露 API + 1.0.0 + deprecated 过渡
用户：合理版本范围 + 定期 MAJOR 升级规划
```

---

## 3. 重点结论与实用要点

### Crate 作者

| 建议 | 说明 |
|------|------|
| **少暴露** | 公共 API 越小，break 越少（Item 22） |
| **别困在 `0.x`** | API 稳定就发 **1.0.0**，恢复 MAJOR/MINOR/PATCH 三级语义 |
| **平滑过渡** | MINOR 里 `#[deprecated]` + 迁移指南 → 下一 MAJOR 再删 |
| **break 要编译期可见** | 逻辑不兼容时改**类型签名**，让编译器帮用户拦 |

### Crate 用户

| 建议 | 说明 |
|------|------|
| **禁 `"*"` / `"0.*"`** | 可能拉到 break 版；crates.io **拒绝**带 `"*"` 的发布包 |
| **别永远 ignore MAJOR** | 大版本固定安全，但旧库可能缺安全修复；`cargo update` / Dependabot |

### 版本范围语法

| 写法 | 含义 |
|------|------|
| `"1.2.3"` | 等同 `^1.2.3` → `>=1.2.3, <2.0.0` |
| `"=1.2.3"` | 精确锁定（一般不推荐） |
| `"~1.2.3"` | 仅 PATCH → `>=1.2.3, <1.3.0` |

---

## 4. 案例与代码要点

### 破坏性变更速查（需升 MAJOR）

| 变更 | 为何 break |
|------|------------|
| 向 enum **加变体**（非 `#[non_exhaustive]`） | 外部 `match` 未穷尽 |
| 向 struct **加字段**（可外部构造） | 字面量/构造报错 |
| trait **失去 object safety** | `dyn Trait` 用法失效 |
| 已有 trait **加 blanket impl** | 与手动 impl 冲突 |
| 改 **License** / **MSRV** | 广义 API 契约变化 |
| 移除或改 **默认 features** | 下游行为突变 |

### `deprecated` 过渡示例

```rust
#[deprecated(since = "1.3.0", note = "use new_api instead")]
pub fn old_api() { /* ... */ }

pub fn new_api() { /* ... */ }
```

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **「兼容」加 API 仍可能伤用户** | 新函数/类型/带默认的 trait 方法通常算 MINOR；但若用户 `use crate::*`，可能**命名冲突**（Item 23） |
| **库 crate 提交 lock 无助于下游** | 下游构建**忽略**库的 `Cargo.lock`；lock 只为**本仓库 CI** —— 须配 Dependabot，否则间接依赖冻结 |
| **Hyrum 定律** | 内部「无害」改动也可能 break 依赖未文档化行为的用户 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 21](../ER-拓展索引.md#item-21)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 三段 | MAJOR break / MINOR 加功能 / PATCH 修 bug |
| `0.x` | 左移规则：最左非零位变 = break |
| 作者 | 少暴露、敢 1.0、deprecated 过渡 |
| 用户 | 禁 `*`、定期规划 MAJOR |
| `^1.2.3` | 默认：`<2.0.0` |
| lock | 库 lock 不保护下游；CI 要自动更新 |
