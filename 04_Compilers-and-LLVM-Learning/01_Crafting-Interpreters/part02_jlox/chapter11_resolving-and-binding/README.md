# 第 11 章 · Resolving and Binding（解析与绑定）

> **Crafting Interpreters** · [Part II · jlox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/resolving-and-binding.html) · [中文在线](https://craftinginterpreters.com/resolving-and-binding.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

ch10 闭包靠 环境链 + 按名向上查找 能跑通多数情况，但存在 词法作用域 Bug（外层重新 `var` 后，闭包可能绑错变量）。ch11 引入 语义分析 (Semantic Analysis)：运行前 解析每个变量「在第几层环境」，运行时 按 distance 直取。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §11.1 | 静态作用域（Static Scope） | [01-static-scope.md](./01-static-scope.md) |
| §11.2 | 语义分析（Semantic Analysis） | [02-semantic-analysis.md](./02-semantic-analysis.md) |
| §11.3 | 解析器类（A Resolver Class） | [03-a-resolver-class.md](./03-a-resolver-class.md) |
| §11.4 | 解释已解析的变量（Interpreting Resolved Variables） | [04-interpreting-resolved-variables.md](./04-interpreting-resolved-variables.md) |
| §11.5 | 解析错误（Resolution Errors） | [05-resolution-errors.md](./05-resolution-errors.md) |
| ·7 | Resolver 与 Interpreter 分工 | [06-resolver-interpreter.md](./06-resolver-interpreter.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
