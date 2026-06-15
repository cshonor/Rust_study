# 02 · Data Representation in Rust

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] 已读（笔记整理）
- [x] 示例 crate（repr / DST / ZST / niche）

---

## 一句话

**布局章** — 默认 `repr(Rust)` 的对齐、重排与 niche 优化；DST / ZST / 空类型；`repr(C)` / `transparent` / `packed` / `align` 等替代布局。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位与要点 | [00-overview.md](./00-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/repr_rust.rs](./src/repr_rust.rs) | `repr(Rust)` vs `repr(C)`、niche |
| [src/exotic.rs](./src/exotic.rs) | DST 胖指针、ZST、Void |
| [src/repr_alt.rs](./src/repr_alt.rs) | `repr(C)` / `transparent` / `u8` / `packed` / `align` |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/02_Data_Layout
cargo run
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| 对齐与三种 repr | [RFR 02-layout](../../02-RFR/Chapter-02-Types/02-layout.md) |
| 实测输出 | [layout-demo](../../02-RFR/Chapter-02-Types/layout-demo/) |
| 上一章 | [01_Safe_Unsafe](../01_Safe_Unsafe/README.md) |
| 下一章 | [03_Lifetime_Variance](../03_Lifetime_Variance/README.md) · 生命周期 |

---

## 逻辑脉络

默认布局与 niche → 非常规尺寸类型 → 按需选用 `repr` → 进入 Ownership and Lifetimes。
