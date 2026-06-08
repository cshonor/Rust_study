# Item 1 · 01 基础类型严谨性

← [Item 1 目录](../README.md)

> **总原则（背一句）**：强静态类型 · 编译期检查 · **无隐式数值转换** · **无 null**（用 `Option`）。

## 定义

Rust 是**强静态类型**语言，所有基础类型有**明确位数与符号**，类型安全由编译器强制保证。

## 整数类型

| 分类 | 类型 |
|------|------|
| 固定大小 | `i8` / `i16` / `i32` / `i64` / `i128`，`u8` / `u16` / `u32` / `u64` / `u128` |
| 平台相关 | `isize` / `usize`（等于指针宽度，32 或 64 位） |

## 关键规则

- ❌ **无隐式类型转换**（`u8 → u32` 也必须 `as` 或 `From`/`Into`）
- ❌ 不同类型不能直接比较、赋值
- ✅ 必须显式处理截断、溢出风险（`as` 截断；溢出 debug 下 panic，可用 `wrapping_*` / `checked_*` 等）

```rust
let a: u8 = 10;
let b: u32 = a as u32; // 必须显式
// let c: u32 = a;     // ❌ mismatched types

let x: i32 = 42;
let y: i16 = x; // ❌ mismatched types
```

## 面试补一句

「无隐式转换」指数值/基础类型层面；另有 Deref 强制转换、trait 对象等，别和整数混谈。

## 相关

- 标量补充 → [02-scalar-types.md](./02-scalar-types.md)
- 背诵 → [cheat-sheet.md](./cheat-sheet.md)
