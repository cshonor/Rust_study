# 10 · Beneath std (no_std)

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md) · 全书收官

## 状态

- [x] 已读（笔记整理）
- [x] 示例 crate（`#![no_std]` 库 + `panic_handler` + no_main 模板）

---

## 一句话

**no_std 收官章** — 脱离 std 后接管入口、panic、libc/compiler_builtins；stable 可建 no_std 库，裸机 exe 通常需 nightly。

---

## 专项笔记

| 节 | 主题 | 阅读 |
|:--:|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| 1 | 引入 libc 依赖 | [01-libc.md](./01-libc.md) |
| 2 | 无 std 可执行程序 | [02-no-main.md](./02-no-main.md) |
| 3 | 自定义 panic 处理 | [03-panic-handler.md](./03-panic-handler.md) |
| — | no_main 裸机模板 | [templates/no_main_linux.md](./templates/no_main_linux.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/lib.rs](./src/lib.rs) | `#![no_std]` + `#[panic_handler]` |
| [src/main.rs](./src/main.rs) | std 宿主调用 no_std 库 |
| [templates/no_main_linux.md](./templates/no_main_linux.md) | `_start` / `eh_personality` 参考 |

```bash
cd 03-Rust_Nomicon/10_NoStd
cargo run                              # std 宿主 demo
cargo build --no-default-features      # 纯 no_std 库
cargo test                             # std 下单元测试
```

**libc 依赖示例**（注释于 `Cargo.toml`）：

```toml
# libc = { version = "0.2", default-features = false }
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| ER no_std | [Item 33 demo](../../01-ER/Chapter-06-Beyond-Standard-Rust/Item-33-no-std/demo/) |
| OBRM / panic | [06_OBRM_RAII](../06_OBRM_RAII/README.md) |
| FFI 入口 | [09_FFI](../09_FFI/README.md) |
| 上一章 | [09_FFI](../09_FFI/README.md) |

---

## 逻辑脉络

libc 不拉回 std → no_main 入口 → panic_handler → 内核/嵌入式实战。
