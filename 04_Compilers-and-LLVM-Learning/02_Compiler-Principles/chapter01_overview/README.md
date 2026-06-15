# 第 1 章 · 编译总览（Overview of Compilation）

> **Engineering a Compiler 3e** · [02 编译器工程](../../README.md) · [本书目录](../../本书目录.md)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

第一章不写具体算法，而是搭建**编译器工程总框架**：编译器是什么、必须守什么原则、典型三阶段结构、一次翻译经历哪些步骤、好编译器还应具备什么工程性质——本质是**正确性、实用性与各种 Trade-off 之间的权衡艺术**。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 基本概念与研究动机 | [01-basic-concepts-and-motivation.md](./01-basic-concepts-and-motivation.md) |
| §2 | 两大基本原则 | [02-two-fundamental-principles.md](./02-two-fundamental-principles.md) |
| §3 | 三阶段结构（前端 / 优化器 / 后端） | [03-three-phase-structure.md](./03-three-phase-structure.md) |
| §4 | 翻译过程关键步骤（含示例语句） | [04-translation-pipeline-example.md](./04-translation-pipeline-example.md) |
| §5 | 编译器应有的工程性质 | [05-desired-properties.md](./05-desired-properties.md) |
| — | 速记 · 自测 · 对照 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch1 | 对照 |
|----------|------|
| 三阶段 / IR | [CI ch2 · Rust/HFT 流水线](../../01_Crafting-Interpreters/part01_welcome/chapter02_map-of-the-territory/04-rust-hft-编译流水线对照.md) |
| 优化 / 寄存器分配 | [04 Learn LLVM 17](../../../04_Learn-LLVM-17/README.md) |
| Rust 编译链 | RFR [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md) |

---

## 逻辑脉络

按上表顺序阅读；`cheat-sheet.md` 含三句背诵与自测。全书 13 章四大部分 → [本书目录](../../本书目录.md)
