# 第 23 章 · Jumping Back and Forth（来回跳转）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/jumping-back-and-forth.html) · [中文在线](https://craftinginterpreters.com/jumping-back-and-forth.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

字节码是 一维扁平指令流 → 控制流 = 改 `ip`（跳转）。本章实现 `if` / `and`·`or` / `while` / `for`，并引入经典 回填 (Backpatching)。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §23.1 | If 语句（If Statements） | [01-if-statements.md](./01-if-statements.md) |
| §23.2 | 逻辑运算符（Logical Operators） | [02-logical-operators.md](./02-logical-operators.md) |
| §23.3 | While 循环（While Statements） | [03-while-statements.md](./03-while-statements.md) |
| §23.4 | For 循环（For Statements） | [04-for-statements.md](./04-for-statements.md) |
| ·6 | 跳转指令族（小结） | [05-jumping-back-and-forth.md](./05-jumping-back-and-forth.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
