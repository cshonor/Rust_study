# 05 · Working With Uninitialized Memory

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] 已读（笔记整理）
- [x] 示例 crate（checked / drop flags / MaybeUninit / ptr）

---

## 一句话

**未初始化内存章** — Safe 分支分析 vs 逻辑 move-out、Drop flags、MaybeUninit 标准路径、`ptr::write` / `copy` 底层操作。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位与要点 | [00-overview.md](./00-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/checked.rs](./src/checked.rs) | 编译期受检的条件初始化 |
| [src/drop_flags.rs](./src/drop_flags.rs) | 条件 Drop（drop flags 直觉） |
| [src/maybe_uninit.rs](./src/maybe_uninit.rs) | `MaybeUninit` 数组、Vec `set_len` |
| [src/ptr_ops.rs](./src/ptr_ops.rs) | `write` / `copy` / `copy_nonoverlapping` |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/05_Uninit_Mem
cargo run
cargo run -- --drop-flags   # 观察条件 Drop 输出
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| MaybeUninit | [RFR 03-calling-unsafe](../../02-RFR/Chapter-09-Unsafe-Code/03-calling-unsafe-functions.md) |
| ptr::write | [Book 19.1 unsafe](../../00-Book/19-advanced-features/19.1-不安全Rust.md) |
| 上一章 | [04_Type_Cast](../04_Type_Cast/README.md) |
| 下一章 | [06_OBRM_RAII](../06_OBRM_RAII/README.md) · OBRM |

---

## 逻辑脉络

Safe 受检未初始化 → Drop flags → MaybeUninit / ptr 突破 Safe 限制 → 进入 OBRM。
