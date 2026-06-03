# Item 35: Prefer bindgen to manual FFI mappings

> **Effective Rust** · [Chapter 6 — Beyond Standard Rust](../ER-本书目录.md)  
> **中文**：优先使用 bindgen 而非手动编写 FFI 映射  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-35-bindgen](../ER-demos/item-35-bindgen/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `unsafe`、FFI 原则 | [19.1 不安全 Rust](../../Book/19-advanced-features/19.1-不安全Rust.md) |
| 发布与构建 | [14.1 发布配置](../../Book/14-cargo-crates/14.1-采用发布配置自定义构建.md) |
| FFI 边界 | [Item 34](./Item-34-ffi-boundaries.md) |
| 避免手写 unsafe | [Item 16](../Chapter-03-Concepts/Item-16-avoid-unsafe.md) |
| CI 校验生成物 | [Item 32](../Chapter-05-Tooling/Item-32-ci.md) |
| `cxx` / 工具生态 | [Item 31](../Chapter-05-Tooling/Item-31-tooling-ecosystem.md) |

---

## 1. 核心知识点与关键定义

### 手工 FFI 的风险（Item 34）

- 手写 `extern "C"` / `#[repr(C)]` → **链接器不验证** C 与 Rust 声明一致。
- 略有不匹配 → **静默** → 运行时崩溃。

### `bindgen`

- 解析 **C 头文件** → 自动生成 Rust `extern` / struct / enum 等声明。
- 与 C 编译器共用**同一份 `.h`** → 接口同步链。

### `cxx`

- Rust ↔ **C++** 互操作；从共享 **schema** 同时生成两侧绑定。
- 比 bindgen 扫 C++ 更**类型安全**、更完整。

---

## 2. 逻辑脉络

```text
C 源码 ↔ C 头文件（C 编译器检查）
         ↓
bindgen(同一 .h) → Rust 声明
         ↓
人工手写 Rust 声明 → 易 drift，无检查
         ↓
CI：重新 bindgen → git diff 与签入版不一致则 fail（Item 32）
         ↓
xyzzy-sys（unsafe 生成码）+ xyzzy（safe 封装）→ Item 16
```

---

## 3. 重点结论与实用要点

### 默认：用 bindgen，别手写

- 除**极简**一两个符号外，一律机器生成。

### `-sys` + 安全层双层架构

| Crate | 职责 |
|-------|------|
| **`xyzzy-sys`** | `bindgen` 输出；**全 unsafe** |
| **`xyzzy`** | 封装为 safe Rust API；业务只依赖此层 |

→ 应用代码继续遵守 Item 16。

### C++ → `cxx`，不是 bindgen

- bindgen 对 C++ **子集有限**；紧密 C++/Rust 互操作用 **`cxx`**。

---

## 4. 案例与代码要点

### Allowlist 提取大库子集

```bash
bindgen --allowlist-function="add.*" \
        --allowlist-type=FfiStruct \
        -o src/generated.rs \
        lib.h
```

### 引入生成文件

```rust
include!("generated.rs");
```

或在 **`build.rs`** 里构建时生成（§6）。

### `build.rs` 骨架（待深入）

```rust
// build.rs — 概念示例
let bindings = bindgen::Builder::default()
    .header("lib.h")
    .allowlist_function("add.*")
    .generate()?;
bindings.write_to_file(out_dir.join("generated.rs"))?;
```

`cargo build` 时扫描本地头文件 → 适应跨平台路径。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **手写 struct 字段顺序/类型错** | 编译链接**仍成功** → 内存错乱 |
| **改 C API 未重跑 bindgen** | 签入旧 `generated.rs` → CI diff 应拦截 |
| **把 bindgen 输出当 safe 用** | `-sys` 层必须再包一层 |
| **C++ 大项目硬 bindgen** | 模板/重载/命名空间 → 用 **cxx** |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 35](../ER-拓展索引.md#item-35)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 原则 | **bindgen > 手写** |
| 同步 | 与 C 共用 **同一 .h** |
| 架构 | **`foo-sys`** + **`foo`** safe |
| C++ | **`cxx`** |
| CI | 重生 bindgen → **diff fail** |
| 大库 | **allowlist** 只取需要的 API |

---

> **Effective Rust 全书 35 Items 笔记整理完成。**
