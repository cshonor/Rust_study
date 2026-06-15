# 第 21 章 · Global Variables（全局变量）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/global-variables.html) · [中文在线](https://craftinginterpreters.com/global-variables.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

ch20 哈希表 落地为 `vm.globals` → Lox 终于有了 状态。同时引入 语句、`print`、`var`，以及 Pratt 中的 赋值优先级 / `canAssign`。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| ·2 | 语句与状态效果（Statements） | [01-statements.md](./01-statements.md) |
| ·3 | 全局变量声明（Variable Declarations） | [02-variable-declarations.md](./02-variable-declarations.md) |
| ·4 | 读取和赋值（Reading and Assignment） | [03-reading-and-assignment.md](./03-reading-and-assignment.md) |
| ·5 | 赋值的优先级（Assignment Precedence） | [04-assignment-precedence.md](./04-assignment-precedence.md) |
| ·6 | 全局变量管线小结 | [05-global-variables.md](./05-global-variables.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
