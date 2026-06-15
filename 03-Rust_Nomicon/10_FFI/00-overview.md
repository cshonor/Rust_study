# 10 · Foreign Function Interface · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[09 Vec/Arc](../09_Impl_Vec_Arc/README.md) · 下一章：[11 NoStd](../11_NoStd/README.md)

---

官方标题 **Foreign Function Interface (FFI)**。探讨 Rust 与外部环境（主要为 **C**）如何安全、高效互调。C 是系统编程的「通用语」，本章是打破语言壁垒的**避坑指南**。

| 对照 | 路径 |
|------|------|
| RFR FFI 章 | [Ch11 FFI](../../02-RFR/Chapter-11-Foreign-Function-Interfaces/README.md) |
| ER Item 34 | [ffi-boundaries](../../01-ER/Chapter-06-Beyond-Standard-Rust/Item-34-ffi-boundaries/README.md) |
| repr(C) | [02_Data_Layout](../02_Data_Layout/00-overview.md) |
| catch_unwind | [Book 9.1.3](../../00-Book/09-error-handling/9.1.3-catch_unwind与panic模式.md) |

**读完应能回答**：FFI 边界为何 unsafe、panic 为何不能跨界、如何用 `repr(C)`/`CString`/`Option` niche 安全互操作。

---

## 1. 从 Rust 调用 C (Calling foreign functions)

| 要点 | 说明 |
|------|------|
| 声明 | `extern "C" { fn foo(...); }` + `#[link(name = "...")]` |
| Unsafe | 所有 C 调用 **`unsafe`** — 编译器无法检查 C 的内存/线程安全 |
| 最佳实践 | 用 Rust 类型（`Vec`、切片）**包装**成 Safe API |

→ 源码：[src/call_c.rs](./src/call_c.rs)

---

## 2. 从 C 调用 Rust (Calling Rust from C)

| 要点 | 说明 |
|------|------|
| 调用约定 | `pub extern "C" fn` |
| 符号名 | `#[no_mangle]`（新版亦见 `#[unsafe(no_mangle)]`）禁用名称重整 |
| 产物 | `Cargo.toml` 中 `crate-type = ["cdylib"]` 生成 C 动态库 |

→ 源码：[src/export_to_c.rs](./src/export_to_c.rs)

---

## 3. 回调 (Callbacks)

| 模式 | 说明 |
|------|------|
| 全局回调 | `extern "C" fn` 直接传给 C |
| 带状态 | `*mut RustObject` 传入，回调内 `unsafe` 转回 |
| 异步/异线程 | **极危险** — 须 `Mutex`/通道，销毁前**注销**回调 |

→ 源码：[src/callbacks.rs](./src/callbacks.rs)

---

## 4. 互操作与数据表示 (Interoperability)

| 类型 | 规则 |
|------|------|
| 结构体 | 跨边界须 **`#[repr(C)]`** |
| 字符串 | Rust `str` 无 `\0` → 用 **`CString`** |
| 可变参数 | `extern` 块可**声明** C 的 `...`；Rust **不能定义** variadic fn |

→ 源码：[src/interop.rs](./src/interop.rs)

---

## 5. 外部全局变量 (Foreign globals)

C 库常导出 `static` 全局状态 → Rust 用 `extern { static mut ... }` 声明；**读写均为 unsafe**，极度危险。

→ 注释见 [src/globals.rs](./src/globals.rs)

---

## 6. 可空指针优化 (Nullable pointer optimization)

Rust 无 null；FFI 中 **`Option<extern "C" fn(...)>`** / **`Option<NonNull<T>>`** 的 `None` 即底层 `null`，无额外 tag。

→ 源码：[src/nullable.rs](./src/nullable.rs)

---

## 7. 异常与栈展开 (FFI and unwinding)

**Rust `panic!` 跨入 C，或 C++ 异常跨入 Rust → UB**。

| 对策 | API |
|------|-----|
| 捕获 panic | **`catch_unwind`** 于 FFI 边界 |
| 显式允许展开 | `extern "C-unwind"`（特定场景） |

→ 源码：[src/unwind.rs](./src/unwind.rs)

---

## 8. 不透明结构体 (Opaque structs)

C 只给指针、不公开布局 → Rust 用 **`#[repr(C)]` struct + 私有字段 + `PhantomData`**，优于空枚举或裸 `c_void`（类型更安全）。

→ 源码：[src/opaque.rs](./src/opaque.rs)

---

## 总结

利用 **`repr(C)`、裸指针、`Option` niche、`catch_unwind`** 等机制，在获得底层控制力的同时尽量维护安全边界。
