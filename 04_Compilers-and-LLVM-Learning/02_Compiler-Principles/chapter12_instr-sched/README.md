# 第 12 章 · 指令调度（Instruction Scheduling）

> **Engineering a Compiler 3e** · [02 编译器工程](../../README.md) · [本书目录](../../本书目录.md) · Part IV 代码生成

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**后端第二关** — ch11 选出指令后，在**不改语义**前提下**重排顺序**以隐藏 **stall**（load/FP 延迟）；核心武器 **列表调度**（依赖图 + 就绪队列 + 优先级）；**NP 完全**故用启发式；与 ch13 **寄存器分配**存在 **phase ordering** 冲突；进阶 **区域调度 · 上下文复制**。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 核心问题与动机 | [01-motivation-and-problem.md](./01-motivation-and-problem.md) |
| §2 | 列表调度 | [02-list-scheduling.md](./02-list-scheduling.md) |
| §3 | 调度与寄存器分配的矛盾 | [03-scheduling-vs-regalloc.md](./03-scheduling-vs-regalloc.md) |
| §4 | 高级话题 | [04-advanced-topics.md](./04-advanced-topics.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch12 | 对照 |
|----------|------|
| ch1 后端三难点 | [ch1 §4.3](../chapter01_overview/04-translation-pipeline-example.md) |
| ch11 指令筛选 | [ch11](../chapter11_instr-select/README.md) — 调度在其后 |
| ch9 活跃变量 | [ch9 §1](../chapter09_dataflow/01-iterative-dataflow.md) — 调度增 LIVE → spill |
| ch13 寄存器分配 | 下一章 — phase ordering 核心 |
| HFT 延迟 | 热点路径 stall / spill 极敏感 |
| LLVM | `MachineScheduler` · post-RA / pre-RA 调度 |

---

## 逻辑脉络

动机与 NP 性 → 列表调度 → 与 regalloc 博弈 → 区域扩展。
