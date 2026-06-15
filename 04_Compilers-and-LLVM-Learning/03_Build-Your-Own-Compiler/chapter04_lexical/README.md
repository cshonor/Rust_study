# 第 4 章 · 词法分析

> **《自制编译器》** · [03 Build Your Own Compiler](../../README.md) · [本书目录](../../本书目录.md) · 第1部分 · 代码分析

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**JavaCC 扫描器实战** — 用 **正则** 写 `.jj` 规则：**TOKEN** 产 token（标识符/保留字 **最长匹配** · 二/八/十六进制数）；**SKIP / SPECIAL_TOKEN** 跳过空白与注释；**状态迁移 + MORE** 处理块注释、字符串/字符字面量，避免最长匹配 **过度吞噬**。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 基于 JavaCC 的扫描器描述 | [01-javacc-regex.md](./01-javacc-regex.md) |
| §2 | 扫描没有结构的单词 | [02-token-unstructured.md](./02-token-unstructured.md) |
| §3 | 扫描不生成 token 的单词 | [03-skip-and-special.md](./03-skip-and-special.md) |
| §4 | 扫描具有结构的单词 | [04-structured-lexing-states.md](./04-structured-lexing-states.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch4 | 对照 |
|----------|------|
| ch3 概念 | [chapter03_parse-overview](../chapter03_parse-overview/README.md) · token · JavaCC |
| ch5 下一章 | 解析器 `.jj` 产生式 |
| CI 手写扫描 | [CI ch4 扫描](../../../01_Crafting-Interpreters/part02_jlox/chapter04_scanning/) |
| EaC 正则/FA | [EaC ch2 扫描](../../../02_Compiler-Principles/chapter02_scanners/) |

---

## 逻辑脉络

正则语法 → TOKEN 与字面量 → SKIP → 状态 + MORE 处理结构化词法。
