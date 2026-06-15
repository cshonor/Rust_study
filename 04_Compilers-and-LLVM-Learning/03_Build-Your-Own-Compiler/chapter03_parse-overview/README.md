# 第 3 章 · 语法分析的概要

> **《自制编译器》** · [03 Build Your Own Compiler](../../README.md) · [本书目录](../../本书目录.md) · 第1部分 · 代码分析

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**第1部分开篇** — 代码分析三阶段（**词法 / 语法 / 语义**）；**Scanner → token**（字面+种类+语义值）；**Parser → AST**（节点、省略纯格式符号）；**解析器生成器**（LL vs LALR → 选 **JavaCC**）；**.jj** 文件结构与 `javacc` 工作流。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 语法分析的方法 | [01-analysis-phases-and-tokens.md](./01-analysis-phases-and-tokens.md) |
| §2 | 解析器生成器 | [02-parser-generators.md](./02-parser-generators.md) |
| §3 | JavaCC 的概要 | [03-javacc-overview.md](./03-javacc-overview.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch3 | 对照 |
|----------|------|
| ch2 cbc | [chapter02_cflat-cbc](../chapter02_cflat-cbc/README.md) · `parser` 包 |
| ch4～6 | 词法 · JavaCC 规则 · 完整 parser |
| CI 扫描/语法 | [CI ch4 扫描](../../../01_Crafting-Interpreters/part02_jlox/chapter04_scanning/) · [ch5 文法](../../../01_Crafting-Interpreters/part02_jlox/chapter05_representing-code/) |
| EaC | [ch2 扫描](../../../02_Compiler-Principles/chapter02_scanners/) · [ch3 语法分析](../../../02_Compiler-Principles/chapter03_parsers/) |

---

## 逻辑脉络

三阶段与 token/AST → 为何用生成器 → JavaCC 入门格式。
