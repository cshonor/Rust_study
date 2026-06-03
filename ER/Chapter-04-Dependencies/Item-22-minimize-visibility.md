# Item 22: Minimize visibility

> **Effective Rust** · [Chapter 4 — Dependencies](../ER-本书目录.md)  
> **中文**：最小化可见性  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo / 代码练习

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 模块、默认私有、`pub` | [7.2 定义模块与私有性](../../Book/07-packages-modules/7.2-定义模块来控制作用域与私有性.md) |
| `pub struct` 与字段 | [7.3 路径](../../Book/07-packages-modules/7.3-路径用于引用模块树中的项.md) |
| 模块分文件 | [7.5 模块分割进不同文件](../../Book/07-packages-modules/7.5-将模块分割进不同文件.md) |
| `pub use` 对外 API | [14.2 发布 crate](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| Semver 与 API 变更 | [Item 21](./Item-21-semver.md) |
| 重导出门面 | [Item 24](./Item-24-re-export-api-types.md)（待补） |

---

## 1. 核心知识点与关键定义

### 模块级默认私有

- 可见性基本单位：**module**。
- 默认：项仅对**同模块 + 子模块**可见。

### `pub` 与类型差异

| 类型 | `pub` 的效果 |
|------|----------------|
| **`struct`** | 类型公开；**字段、方法须逐一 `pub`** |
| **`enum`** | 类型 + **所有变体**自动公开 |
| **`trait`** | trait + **所有方法**自动公开 |

### 细粒度修饰符

| 写法 | 可见范围 |
|------|----------|
| （无 / `pub(self)`） | 当前模块 |
| `pub(super)` | 父模块及其子树 |
| `pub(crate)` | 整个 crate |
| `pub(in path)` | 指定祖先路径子树 |
| `pub` | 任意 crate 可访问（**公开 API**） |

---

## 2. 逻辑脉络

```text
默认私有 → 只暴露必要 API
         ↓
pub 扩大可见性 = 消耗未来重构自由（Semver 单向门）
  私有 → 公开：Minor（兼容）
  公开 → 私有：Major（break）
         ↓
字段/内部类型公开 → 换实现 = Major
隐藏细节 → 内部可优化而不 break 用户
```

与 Item 21 联动：**API 面越小，Major 升级压力越小**。

---

## 3. 重点结论与实用要点

### 最小可见性原则

- 非公开 API 的一部分 → **尽量收窄**可见性。
- 与 *Effective Java*（访问器优于公开字段）、*Effective C++*（接口不暴露数据成员）同宗。

### 内部共享：优先 `pub(crate)`

- 多模块共用、但**不属于对外 API** → `pub(crate)`，不要直接 `pub`。

### 封装换自由度

- 公开字段 = 承诺布局；改 `Vec` → `HashMap` 等优化 → **Major**。
- 私有字段 + 方法 = 实现可换。

---

## 4. 案例与代码要点

### struct 字段与方法须逐一公开

```rust
pub mod somemodule {
    #[derive(Debug, Default)]
    pub struct AStruct {
        count: i32,        // 私有
        pub name: String,  // 显式 pub
    }

    impl AStruct {
        fn canonical_name(&self) -> String { /* ... */ } // 私有
        pub fn id(&self) -> String { /* ... */ }          // 公开
    }
}

// 外部：s.count、canonical_name() → private field / private method
```

### `pub(in path)` — std 迭代器适配器

- 内部模块 `std::iter::adapters` 用 **`pub(in crate::iter)`** 限制方法可见性。
- 外层 **`pub use`** 再给用户扁平 API（见 Item 24）。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **外围模块未 `pub`** | `somemodule` 私有 → 其内 `pub` 项外部仍不可达 |
| **`dead_code` 误读** | 「别的模块要用」却忘 `pub` / `pub(crate)` → 实际封闭在当前模块 |
| **`pub struct` ≠ 字段公开** | 常见误以为 struct `pub` 就全公开 |

---

## 6. 后续拓展（待补）

- [ ] `mod.rs` vs `name.rs` 与大型项目目录 + 可见性分层
- [ ] **`pub use` 门面**（Item 24）：对外扁平、对内严格私有

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 默认 | **私有**；要公开才写 `pub` |
| struct | 字段/方法**逐个** `pub` |
| 内部共享 | **`pub(crate)`** |
| Semver | 公开→私有 = **Major** |
| 外层模块 | 父模块也要 `pub` 才能透传 |
| dead_code | 先查可见性修饰符 |
