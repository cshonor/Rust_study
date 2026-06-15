# 第 10 章 · 标量优化（Scalar Optimization）

> **Engineering a Compiler 3e** · [02 编译器工程](../../README.md) · [本书目录](../../本书目录.md) · Part III 优化

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**Part III 实践篇** — 在 ch9 **SSA** 上系统应用标量 Pass：**机器无关/相关**分类法；**死代码 · 代码移动 · 特化 · 冗余消除 · 激活型转换**；进阶 **强度减弱 · Pass 组合 · 优化序列** — 优化器的「兵器谱」，衔接 ch11 代码生成。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 标量转换分类法 | [01-classification.md](./01-classification.md) |
| §2 | 核心优化技术与实例 | [02-core-transformations.md](./02-core-transformations.md) |
| §3 | 高级话题 | [03-advanced-topics.md](./03-advanced-topics.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch10 | 对照 |
|----------|------|
| ch8 优化概述 | [ch8](../chapter08_intro-opt/README.md) — 冗余、作用域直觉 |
| ch9 数据流 / SSA | [ch9](../chapter09_dataflow/README.md) — LIVE、建/拆 SSA |
| LLVM Pass 管线 | [04 ch07 优化](../../../04_Learn-LLVM-17/part02_src_to_machine/chapter07_ir_optimize/) · `optimize_compare/` |
| CI 局部优化 | [clox ch30](../../../01_Crafting-Interpreters/part03_clox/chapter30_optimization/README.md) |
| ch11 指令筛选 | 下一章 — 机器相关后端 |

---

## 逻辑脉络

分类框架 → 五大类核心 Pass → 强度减弱与 Pass 编排。
