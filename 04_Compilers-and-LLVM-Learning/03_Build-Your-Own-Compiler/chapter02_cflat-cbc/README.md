# 第 2 章 · C♭ 和 cbc

> **《自制编译器》** · [03 Build Your Own Compiler](../../README.md) · [本书目录](../../本书目录.md) · 开篇

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**语言 + 工程地图** — **C♭**：保留指针的 C 简化子集，无预处理器，用 **`import`** 替代 `#include`；**cbc**：Java 5 · **11 包**（数据 `ast`/`ir`/`type` … vs 处理 `compiler`/`parser` …）；**`Compiler.build`** 循环 compile → assemble → link；**`compile`** 单文件：parse → 语义 → IR → 汇编。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | C♭ 语言的概要 | [01-cflat-language.md](./01-cflat-language.md) |
| §2 | cbc 的包结构 | [02-cbc-packages.md](./02-cbc-packages.md) |
| §3 | 编译控制与流程 | [03-compiler-control-flow.md](./03-compiler-control-flow.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch2 | 对照 |
|----------|------|
| ch1 四阶段 | [chapter01_start](../chapter01_start/README.md) |
| 第1部分 | ch3～6 代码分析 — 从 `parser` 包深入 |
| 模块 / use | `00-Book` 第 7 章 · Rust `mod`/`use` vs C♭ `import` |
| 多文件编译 | [EaC ch6 过程](../../../02_Compiler-Principles/chapter06_procedures/README.md) |

---

## 逻辑脉络

C♭ 长什么样 → cbc 代码怎么分包 → `build`/`compile` 怎么串起来。
