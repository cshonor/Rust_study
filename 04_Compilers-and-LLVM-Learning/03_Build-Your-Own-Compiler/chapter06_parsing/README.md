# 第 6 章 · 语法分析

> **《自制编译器》** · [03 Build Your Own Compiler](../../README.md) · [本书目录](../../本书目录.md) · 第1部分 · 代码分析

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**C♭ 文法落地** — 自上而下四类单位：**定义**（`compilation_unit` · import · defun/defvars/struct/typedef · **后置类型修饰**）→ **语句**（`stmt` 13 种 · **LOOKAHEAD(1) else**）→ **表达式**（`expr`…`expr1` **分层优先级**）→ **项**（`unary`/`postfix`/`primary`）。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 定义的分析 | [01-definitions.md](./01-definitions.md) |
| §2 | 语句的分析 | [02-statements.md](./02-statements.md) |
| §3 | 表达式的分析 | [03-expressions.md](./03-expressions.md) |
| §4 | 项的分析 | [04-terms.md](./04-terms.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch6 | 对照 |
|----------|------|
| ch4～5 | [词法](../chapter04_lexical/) · [EBNF/LOOKAHEAD](../chapter05_javacc-parser/) |
| ch7 下一部分 | JavaCC **action** 建 AST |
| CI | [ch6～7 解析/表达式](../../../01_Crafting-Interpreters/part02_jlox/) |
| EaC | [ch3 语法分析](../../../02_Compiler-Principles/chapter03_parsers/) |

---

## 逻辑脉络

文件/定义 → 语句 → 表达式优先级塔 → 一元/后缀/primary。
