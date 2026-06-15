# 第 2 章 · 扫描（Scanners / 词法分析）

> **Engineering a Compiler 3e** · [02 编译器工程](../../README.md) · [本书目录](../../本书目录.md) · Part I 前端

## 状态

- [x] 已读（笔记整理）

---

## 一句话

第 1 章是宏观总览；**第 2 章正式踏入前端第一站** — 从源代码**字符流**中提取符合语言规则的「单词」（**Token**），并确定词性。核心链路：**正则表达式 → NFA → DFA → 最小化 DFA → 表驱动/直接编码实现**，辅以**散列表关键字识别**等工程权衡。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 扫描器的核心任务 | [01-scanner-core-tasks.md](./01-scanner-core-tasks.md) |
| §2 | 正则表达式（RE） | [02-regular-expressions.md](./02-regular-expressions.md) |
| §3 | 有穷自动机（NFA / DFA） | [03-finite-automata.md](./03-finite-automata.md) |
| §4 | 自动生成管线 | [04-re-to-dfa-pipeline.md](./04-re-to-dfa-pipeline.md) |
| §5 | 工程实现 | [05-scanner-implementation.md](./05-scanner-implementation.md) |
| §6 | 关键字识别与权衡 | [06-keywords-and-tradeoffs.md](./06-keywords-and-tradeoffs.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch2 | 对照 |
|----------|------|
| 手写扫描器 | [CI jlox ch4 Scanning](../../../01_Crafting-Interpreters/part02_jlox/chapter04_scanning/README.md) |
| 按需扫描 | [CI clox ch16 Scanning on Demand](../../../01_Crafting-Interpreters/part03_clox/chapter16_scanning-on-demand/README.md) |
| 流水线位置 | [CI ch2 · 上山前端](../../../01_Crafting-Interpreters/part01_welcome/chapter02_map-of-the-territory/04-rust-hft-编译流水线对照.md) |
| ch1 三阶段 | [ch1 §3 前端](../chapter01_overview/03-three-phase-structure.md) |

---

## 逻辑脉络

RE 描述「找什么」→ FA 描述「怎么找」→ 构造算法 → 代码形态 → 关键字工程技巧。
