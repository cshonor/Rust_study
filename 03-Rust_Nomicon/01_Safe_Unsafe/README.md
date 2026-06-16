# 01 · Meet Safe and Unsafe

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md) · 开篇

## 状态

- [x] 已读（笔记整理）
- [x] 深度总结（Safe/Unsafe · HFT 适配）
- [x] 示例 crate（五种 unsafe 能力 + privacy 封装）

---

## 一句话

**心智模型章** — Safe / Unsafe 双重世界、`unsafe` 契约、五种额外能力、信任不对称、安全性非局部性；为全书 unsafe 编程定调。

---

## 专项笔记

| 节 | 主题 | 阅读 |
|:--:|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| 一 | 为何分 Safe/Unsafe | [01-why-safe-unsafe.md](./01-why-safe-unsafe.md) |
| 二 | unsafe 两种作用 | [02-unsafe-contract.md](./02-unsafe-contract.md) |
| 三 | 五种高危能力 | [03-five-powers.md](./03-five-powers.md) |
| 四 | 信任与非局部性 | [04-trust-and-nonlocality.md](./04-trust-and-nonlocality.md) |
| 五 | 易错疑问 | [05-faq.md](./05-faq.md) |
| 六 | HFT 实操规范 | [06-hft-practice.md](./06-hft-practice.md) |
| 七 | 裸指针完整解读 | [07-raw-pointers.md](./07-raw-pointers.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/five_powers.rs](./src/five_powers.rs) | 五种 unsafe 能力 |
| [src/raw_pointers.rs](./src/raw_pointers.rs) | `*const` / `*mut` 创建 vs 解引用 |
| [src/privacy.rs](./src/privacy.rs) | 模块边界封装 invariant |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/01_Safe_Unsafe
cargo run
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| unsafe 入门 | [The Book 19.1](../../00-Book/19-advanced-features/19.1-不安全Rust.md) |
| 深入 | [RFR Ch09 Unsafe](../../02-RFR/Chapter-09-Unsafe-Code/README.md) |
| Miri 验证 | [ER Item 16 demo](../../01-ER/Chapter-03-Concepts/Item-16-avoid-unsafe/demo/) |
| 下一章 | [02_Data_Layout](../02_Data_Layout/README.md) · 数据布局 |

---

## 逻辑脉络

设计出发点 → 契约机制 → 五种能力对照源码 → 信任与非局部性 → FAQ → HFT 规范 → 进入 Data Layout。
