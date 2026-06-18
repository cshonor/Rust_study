# 3.2 Build Configuration（构建配置）

> 所属：**Project Configuration** · [← 章索引](./README.md)

← [04 Crate 元数据](./04-crate-metadata.md) · 下一节 [06 条件编译](./06-conditional-compilation.md)

前置 → [03 工作区](./03-workspaces.md)（根 profile / patch 仅根生效）

Book → [14.1 发布配置](../../00-Book/14-cargo-crates/14.1-采用发布配置自定义构建.md)

---

## 章节总览

| 模块 | 作用 |
|------|------|
| **`[patch]`** | 临时替换依赖（上游 bug 修复） |
| **`[profile.*]`** | 编译优化权衡（opt-level / codegen-units / lto / panic） |
| **Workspace 继承** | 根 profile 全局 · `inherits` 自定义 profile |

---

## 一、`[patch]` — 临时替换依赖

### 适用场景

第三方库有**未合并 bug** — 本地 fix / fork 验证；**不改动** `[dependencies]`，仅编译期换源；上游合并后**移除 patch**。

### 写法 1：替换 crates.io 包（最常用）

```toml
# 根 Cargo.toml（仅 workspace 根生效）
[patch.crates-io]
serde = { path = "../serde-fix" }
tokio = { git = "https://github.com/xxx/tokio-fix", branch = "bugfix" }
```

### 写法 2：替换 Git 来源依赖

```toml
[dependencies]
foo = { git = "https://github.com/orig/foo" }

[patch."https://github.com/orig/foo"]
foo = { path = "./local-fix-foo" }
```

### 关键约束

| # | 约束 |
|:-:|------|
| 1 | **仅工作区根**生效 — 子 crate 内 `[patch]` **被忽略** |
| 2 | **临时开发** — 发布前删除，**禁止**带着 patch 上线 |
| 3 | 同一包 `path` 与 `git` **互斥** |
| 4 | 工作区内**所有**依赖该包的 member **自动**替换 |

### 与 `[replace]` 区别

| | `[patch]` | `[replace]`（已弃用倾向） |
|---|-----------|---------------------------|
| 粒度 | 覆盖**现有**依赖解析 | 更粗 — 强制全局同名包 |
| 现代用法 | **推荐** | 少用，以 Cargo 文档为准 |

---

## 二、`[profile.*]` — 编译优化权衡

控制 rustc 代码生成 — **编译速度 · 运行性能 · 二进制体积** 三者取舍。

### 1. `opt-level` 优化等级

| 值 | 说明 |
|----|------|
| **`0`** | 无优化 — dev 默认，调试最快 |
| **`1` / `2`** | 轻 / 中度优化 |
| **`3`** | 全量性能 — **release 默认** |
| **`"s"`** | 偏体积，略牺牲性能 |
| **`"z"`** | 极致体积 — 嵌入式 / 静态发布 |

### 2. `codegen-units` 代码生成单元

| 值 | 编译 | 运行性能 |
|----|------|----------|
| **大**（release 默认 16） | 并行多，**快** | 跨单元优化弱 |
| **`1`** | **慢** | 全局优化最强 — 正式发布常用 |

### 3. `lto` 链接期优化

| 值 | 说明 |
|----|------|
| **`false`** | 默认 — 编译快，无跨 crate 内联 |
| **`"thin"`** | 轻量 LTO — 折中 |
| **`true` / `"fat"`** | 全量 LTO — 性能↑ 体积↓，**编译极慢** |

### 4. `panic` 崩溃策略

| 值 | 行为 | 特点 |
|----|------|------|
| **`"unwind"`**（默认） | 栈展开 + `Drop` | 支持 `catch_unwind`；二进制更大 |
| **`"abort"`** | 直接终止，**不**展开、**不**析构 | 体积小 — 嵌入式 / 生产服务常用 |

> **注意**：test / bench / build 脚本常**忽略** `panic = "abort"` — 测试需 unwind。团队须**全局统一**约定。

### Release 生产模板

```toml
[profile.release]
opt-level = 3
codegen-units = 1
lto = true
panic = "abort"
strip = "symbols"   # 剥离调试符号
```

→ HFT / 性能向：[Ch02 §05.4](../Chapter-02-Types/05-4-selection-hft.md)

---

## 三、Workspace Profile 继承

### 1. 根 profile 全局生效

**`[profile.*]` 仅根 `Cargo.toml`** — 子 crate 内 profile **被忽略**（与 patch 同规则）。

### 2. `inherits` 自定义 profile

```toml
# CI：继承 release，关 LTO 加快编译
[profile.ci]
inherits = "release"
lto = false
panic = "unwind"   # 测试需要栈展开
```

```bash
cargo build --profile ci
```

### 3. 继承链

```text
内置默认 → inherits 父 profile → 当前字段覆盖（后者赢）
```

→ [03 工作区 §三](./03-workspaces.md)

---

## 四、核心速记

1. **`[patch]`** — 仅根生效；临时修上游；上线前删；path / git 二选一。  
2. **`opt-level`** — 0 调试 · 3 速度 · s/z 体积。  
3. **`codegen-units`** — 大=编译快性能弱 · 1=慢但峰值性能。  
4. **`lto`** — thin 折中 · fat 换编译时间换性能。  
5. **`panic = abort`** — 小体积；测试仍 unwind。  
6. **Profile** — 根统一管理 · `inherits` 复用。

→ 速记：[05-cheat-sheet.md](./05-cheat-sheet.md) · 下一节：[06 条件编译](./06-conditional-compilation.md)
