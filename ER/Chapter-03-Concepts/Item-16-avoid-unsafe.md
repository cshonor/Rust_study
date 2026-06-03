# Item 16: Avoid writing unsafe code

> **Effective Rust** · [Chapter 3 — Concepts](../ER-本书目录.md)  
> **中文**：避免编写不安全代码  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo / 代码练习

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `unsafe` 五类超能力、封装原则 | [19.1 不安全 Rust](../../Book/19-advanced-features/19.1-不安全Rust.md) |
| 智能指针内部实现 | [15 章](../../Book/15-smart-pointers/)、[Item 8](../Chapter-01-Types/Item-08-references-pointers.md) |
| `Send`/`Sync` 与并发 | [16.4 Send 与 Sync](../../Book/16-fearless-concurrency/16.4-Send与Sync.md) |
| FFI 边界（后续） | [Item 34](../Chapter-06-Beyond-Standard-Rust/Item-34-ffi-boundaries.md)、[Item 35](../Chapter-06-Beyond-Standard-Rust/Item-35-bindgen.md) |

---

## 1. 核心知识点与关键定义

### 不安全模式（Unsafe Rust）

- `unsafe` 标记的代码块是**安全 Rust 的超集**。
- 允许常规 Rust 禁止的操作（如**裸指针** `*const T` / `*mut T`），**绕过借用检查器**对裸指针的约束。
- **不等于无规则**：安全不变量仍须由程序员证明并维护。

### 安全责任转移

```text
安全 Rust：编译器证明内存安全
unsafe：     证明责任 → 程序员
```

### FFI 边界

- 与 C 等外部语言交互的调用**天然 unsafe** —— Rust 无法验证对方代码的内存行为。

---

## 2. 逻辑脉络

```text
Rust 卖点：零开销 + 编译期内存安全
         ↓
底层需求：OS API、硬件、极限性能 → 需要「逃生舱」unsafe
         ↓
Item 16 立场：避免 **编写 (writing)**，优先 **复用 (reuse)**
         ↓
std / crates.io 里已有专家封装 → 用安全 API，别重写 unsafe
```

---

## 3. 重点结论与实用要点

### 默认策略：极其克制

遇到「好像只能 unsafe」的问题 → **先查 std / crates.io**，多半已有安全封装。

### 必须写 unsafe 时的防御准则（*Hic sunt dracones*）

| 准则 | 做法 |
|------|------|
| **Safety Comments** | 写清 `unsafe` 依赖的先决条件与不变量（Clippy 会提醒） |
| **最小爆炸半径** | 缩小 `unsafe { }` 范围；开 `unsafe_op_in_unsafe_fn`，`unsafe fn` 内也显式写块 |
| **加倍测试** | 边界、异常路径、回归 |
| **Miri** | 解释 MIR，抓静态分析抓不到的 **UB** |
| **安全封装隔离** | unsafe 在内层；对外只暴露 safe wrapper API |

---

## 4. 案例与代码要点

### 标准库：内部 unsafe、对外 safe

| 类别 | 例子 |
|------|------|
| 智能指针 | `Rc` / `RefCell` / `Arc` — 内部裸指针 |
| 同步原语 | `Mutex` / `RwLock` — OS 级调用 |
| 内存原语 | `Pin`、`Cow`、`mem::take` / `swap` / `replace` |

### crates.io 优质替代（优先复用）

| crate | 用途 |
|-------|------|
| `once_cell` / `std::sync::OnceLock` | 单次初始化全局 |
| `rand` | OS / 硬件随机源 |
| `byteorder` | 字节序 ↔ 数值 |
| `cxx` | Rust ↔ C++ 互操作（类型安全边界） |

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **并发 + unsafe** | 借用检查不保护裸指针；data race 责任全在你 |
| **FFI 想当然** | C 指针生命周期、对齐、ABI 与 Rust 引用**不能默认一致** |
| **大块 unsafe** | 安全逻辑混进 unsafe 块 → 难以审计、难以 Miri |

---

## 6. 后续拓展（待补）

- [ ] **Miri + CI**：集成方式；Strict Provenance 违规案例
- [ ] **大规模 C/C++ 迁移**：结合 Item 34 / 35，`bindgen` + `cxx` 建 FFI 隔离层

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 立场 | **复用 > 重写** unsafe |
| 责任 | 编译器不管 → 你证明不变量 |
| 封装 | unsafe 在内，safe API 在外 |
| 工具 | Safety comment + Miri + 更多测试 |
| FFI | 一律当不可信；见 Item 34/35 |
