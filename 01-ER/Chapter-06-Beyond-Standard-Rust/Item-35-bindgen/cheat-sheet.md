# Item 35 · 记忆卡片

← [Item 35 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 原则 | **bindgen > 手写** |
| 同步 | 与 C 共用 **同一 .h** |
| 架构 | **`foo-sys`** + **`foo`** safe |
| C++ | **`cxx`** |
| CI | 重生 bindgen → **diff fail** |
| 大库 | **allowlist** 只取需要的 API |

---

> **Effective Rust 全书 35 Items 笔记整理完成。**
