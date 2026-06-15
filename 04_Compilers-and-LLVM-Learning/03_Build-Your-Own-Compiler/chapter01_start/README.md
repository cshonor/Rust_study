# 第 1 章 · 开始制作编译器

> **《自制编译器》** · [03 Build Your Own Compiler](../../README.md) · [本书目录](../../本书目录.md) · 开篇

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**全书地图章** — 目标：用 **Java/JavaCC** 实现 **cbc**，把 **C♭** 编译为 **x86 Linux ELF**；厘清 **Build 四步**（预处理→编译→汇编→链接）与 **cbc 内四步**（语法→语义→IR→代码生成）+ 优化； **`cbc hello.cb` → `./hello`** 建立可执行文件直觉。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 本书的概要 | [01-book-overview.md](./01-book-overview.md) |
| §2 | 编译的 4 个主要阶段 | [02-four-compiler-stages.md](./02-four-compiler-stages.md) |
| §3 | 使用 C♭ 编译器进行编译 | [03-cbc-demo.md](./03-cbc-demo.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch1 | 对照 |
|----------|------|
| 三阶段 / 流水线 | [CI ch2 编译之山](../../../01_Crafting-Interpreters/part01_welcome/chapter02_map-of-the-territory/04-rust-hft-编译流水线对照.md) |
| 编译总览 | [EaC ch1](../../../02_Compiler-Principles/chapter01_overview/README.md) |
| ch2 下一章 | [chapter02_cflat-cbc/](../chapter02_cflat-cbc/README.md) · C♭ 语法与 cbc 结构 |
| Rust 编译链 | RFR [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md) |

---

## 逻辑脉络

概要与环境 → cbc 内部分阶段 → 动手跑通 Hello World。
