# 10 · Beneath std · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[09 FFI](../09_FFI/README.md)

---

官方标题 **Beneath std**。探讨 **`#![no_std]`** 环境下须手动接管的底层机制——OS 内核、嵌入式等场景的关键一章。

| 对照 | 路径 |
|------|------|
| ER Item 33 | [no-std](../../01-ER/Chapter-06-Beyond-Standard-Rust/Item-33-no-std/README.md) |
| RFR no_std | [Ch12 Without std](../../02-RFR/Chapter-12-Rust-Without-Standard-Library/README.md) |
| panic_handler | [09_FFI](../09_FFI/00-overview.md) §7 |

**读完应能回答**：为何 `libc` 要关 default-features、`no_main` 做什么、`#[panic_handler]` 签名与唯一性。

---

## 1. 引入 libc 依赖

`#[no_std]` 可执行文件常需 **`libc`**，且必须：

```toml
libc = { version = "...", default-features = false }
```

**`default-features = true` 会隐式拉回 `std`**，破坏 no_std 构建。

→ 见 [00-overview.md](./00-overview.md) 与 [Cargo.toml](./Cargo.toml) 注释

---

## 2. 构建无标准库可执行程序

| 要点 | 说明 |
|------|------|
| **Nightly** | 许多平台须手动提供 **lang items**（如 `eh_personality` 栈展开） |
| **`#![no_main]`** | 禁止编译器生成默认 `main` |
| **入口符号** | 手动 `#[no_mangle] extern "C" fn _start()` / `main` / `WinMain` |
| **`compiler_builtins`** | 缺 `__aeabi_memcpy` 等链接符号时手动链接 |

→ 模板：[templates/no_main_linux.md](./templates/no_main_linux.md)（参考，需 nightly + 目标平台）

---

## 3. 自定义恐慌处理 (`#[panic_handler]`)

无 std 时默认 `panic!` **失效**，须定义：

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { ... }
```

- 签名严格：`fn(&PanicInfo) -> !`
- 整个依赖图**只能有一个**

**多环境策略**（Cargo profile / features）：

| 环境 | 典型 crate |
|------|------------|
| dev 调试 | `panic-semihosting`（输出到主机） |
| release | `panic-halt`（死循环挂起） |

→ 源码：[src/lib.rs](./src/lib.rs)（`panic_halt` 示例）

---

## 本仓库 demo

| 构建 | 命令 |
|------|------|
| 宿主 std 演示 | `cargo run` |
| 纯 `#![no_std]` 库 | `cargo build --no-default-features` |

`no_std` **库**可在 stable 构建；**裸机可执行文件**见 §2 模板（通常 nightly）。
