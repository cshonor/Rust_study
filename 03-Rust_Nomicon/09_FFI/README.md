# 09 · Foreign Function Interface

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] 已读（笔记整理）
- [x] 示例 crate（call C / export / callback / interop / unwind / opaque）

---

## 一句话

**FFI 避坑章** — Rust↔C 互调、`repr(C)`/`CString`、回调与全局变量风险、Option niche、panic 不可跨界、`catch_unwind`、opaque struct。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位与要点 | [00-overview.md](./00-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/call_c.rs](./src/call_c.rs) | 调用 C `abs` + Safe 包装 |
| [src/export_to_c.rs](./src/export_to_c.rs) | `extern "C"` + `#[no_mangle]` 导出 |
| [src/callbacks.rs](./src/callbacks.rs) | 函数指针回调 + `*mut` 状态 |
| [src/interop.rs](./src/interop.rs) | `repr(C)` + `CString` |
| [src/nullable.rs](./src/nullable.rs) | `Option<extern "C" fn>` niche |
| [src/unwind.rs](./src/unwind.rs) | `catch_unwind` 边界 |
| [src/opaque.rs](./src/opaque.rs) | 不透明类型 |
| [src/globals.rs](./src/globals.rs) | 外部 `static mut` 说明 |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/09_FFI
cargo run
cargo test
```

**导出 cdylib**（供 C 链接）：在 `Cargo.toml` 增加 `crate-type = ["cdylib"]` 后 `cargo build`。

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| RFR FFI | [Ch11](../../02-RFR/Chapter-11-Foreign-Function-Interfaces/README.md) |
| bindgen | [ER Item 35](../../01-ER/Chapter-06-Beyond-Standard-Rust/Item-35-bindgen/README.md) |
| 上一章 | [08_Impl_Vec_Arc](../08_Impl_Vec_Arc/README.md) |
| 下一章 | [10_NoStd](../10_NoStd/README.md) |

---

## 逻辑脉络

Rust→C → C→Rust → 类型/字符串 → 回调与全局 → panic 边界 → opaque → no_std。
