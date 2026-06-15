# 第 4 章 · Scanning（扫描 / 词法分析）

> **Crafting Interpreters** · [Part II · jlox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/scanning.html) · [中文在线](https://craftinginterpreters.com/scanning.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

Part II 第一段代码：实现 jlox 扫描器（Scanner）——编译/解释流水线的第一站（[ch02 §2.1.1](../../part01_welcome/chapter02_map-of-the-territory/README.md)）。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §4.1 | 解释器框架（The Interpreter Framework） | [01-the-interpreter-framework.md](./01-the-interpreter-framework.md) |
| §4.2 | 词素与词法单元（Lexemes and Tokens） | [02-lexemes-and-tokens.md](./02-lexemes-and-tokens.md) |
| §4.3 | 正则语言与表达式（Regular Languages and Expressions） | [03-regular-languages-and-expressions.md](./03-regular-languages-and-expressions.md) |
| §4.4 | Scanner 类（The Scanner Class） | [04-the-scanner-class.md](./04-the-scanner-class.md) |
| §4.5 | 识别词素（Recognizing Lexemes） | [05-recognizing-lexemes.md](./05-recognizing-lexemes.md) |
| §4.6 | 更长的词素（Longer Lexemes） | [06-longer-lexemes.md](./06-longer-lexemes.md) |
| §4.7 | 保留字与标识符（Reserved Words and Identifiers） | [07-reserved-words-and-identifiers.md](./07-reserved-words-and-identifiers.md) |
| ·9 | 流水线位置与后续章节 | [08-pipeline.md](./08-pipeline.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
