# 2.2 Calling Unsafe Functions（调用 unsafe 函数）

> 所属：**Great Power** · [← 章索引](./README.md)

## FFI

`extern "C"` 等 — Rust 侧 **`unsafe` 调用**（外部世界不保证 Rust 不变量）。

→ [第 11 章 FFI](../Chapter-11-Foreign-Function-Interfaces/11-外部函数接口-FFI-深度解析.md)

## `_unchecked` API

如 `get_unchecked` — 跳过边界检查；仅在有**可证明**边界 + 可量化热点收益时使用。

## `MaybeUninit<T>`

未初始化存储；**`assume_init`** 等为 `unsafe` — 须证明内存已是**合法 `T` 值**（validity）。

Nomicon → [03-Rust_Nomicon](../../03-Rust_Nomicon/README.md)
