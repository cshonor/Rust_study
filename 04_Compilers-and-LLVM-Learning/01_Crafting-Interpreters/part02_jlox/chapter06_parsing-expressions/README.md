# 第 6 章 · Parsing Expressions（解析表达式）

> **Crafting Interpreters** · [Part II · jlox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/parsing-expressions.html) · [中文在线](https://craftinginterpreters.com/parsing-expressions.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

Part II 第一个重要里程碑：实现 Parser，把 Token 流 → AST（反映语法嵌套）。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §6.1 | 歧义与解析游戏（Ambiguity and the Parsing Game） | [01-ambiguity-and-the-parsing-game.md](./01-ambiguity-and-the-parsing-game.md) |
| §6.2 | 递归下降解析（Recursive Descent Parsing） | [02-recursive-descent-parsing.md](./02-recursive-descent-parsing.md) |
| §6.3 | 语法错误（Syntax Errors） | [03-syntax-errors.md](./03-syntax-errors.md) |
| §6.4 | 组合起来（Wiring up the Parser） | [04-wiring-up-the-parser.md](./04-wiring-up-the-parser.md) |
| ·6 | 流水线里程碑 | [05-parsing-expressions.md](./05-parsing-expressions.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
