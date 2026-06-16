# 三 · Unsafe Rust 独有的 5 种高危能力

← [本章目录](./README.md) · 上一节：[02-unsafe-contract.md](./02-unsafe-contract.md) · 下一节：[04-trust-and-nonlocality.md](./04-trust-and-nonlocality.md)

---

Safe Rust 完全禁止这五类操作；**任意一种违背约束 → 未定义行为**。仅能在 `unsafe {}` 块中执行（实现 unsafe trait 时用 `unsafe impl`）。

| # | 能力 | 要点 |
|:-:|------|------|
| 1 | 解引用裸指针 `*const T` / `*mut T` | 仅**解引用**需 unsafe；定义/赋值裸指针变量属于 Safe |
| 2 | 调用 `unsafe fn` | 含自定义 unsafe 方法、C FFI、`intrinsic` |
| 3 | 实现 `unsafe trait` | 如 `Send`/`Sync`；须 `unsafe impl` |
| 4 | 读写 `static mut` | Safe 中连读都不允许 |
| 5 | 访问 `union` 字段 | 编译器无法跟踪当前有效类型 |

## 1. 解引用裸指针

普通引用 `&T` / `&mut T` 受借用检查、生命周期约束；裸指针完全绕过编译器校验，可指向任意内存地址。

**风险**：悬垂指针、越界访问、同时存在可变 + 不可变访问。

## 2. 调用 unsafe fn

包含三类：自定义不安全方法、C 语言 FFI 外部函数、编译器内置 intrinsic 底层函数。

**规则**：无论函数有无返回值、参数多少，调用时必须包裹 unsafe 块。

## 3. 实现 unsafe trait

普通 trait 实现无需 unsafe；带 unsafe 标记的 trait，实现时强制加 `unsafe impl`。

**风险**：错误实现 `Send`/`Sync` 会引发多线程数据竞争，破坏内存安全。

## 4. 读写 static mut

全局可变静态变量无借用保护、无线程同步，多线程同时读写必然数据竞争。

Safe 代码中连读取 `static mut` 都不允许，只能在 unsafe 块内操作。

## 5. 访问 union 字段

Union 同一内存空间存放多种类型，编译器无法跟踪当前存储的有效类型，直接读取字段极易读取错误二进制数据，引发 UB。

---

→ 源码逐项对照：[src/five_powers.rs](./src/five_powers.rs)

```bash
cd 03-Rust_Nomicon/01_Safe_Unsafe
cargo run
```
