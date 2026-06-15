# 第 11 章 · Resolving and Binding（解析与绑定）

> **Crafting Interpreters** · [Part II · jlox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/resolving-and-binding.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

ch10 闭包靠 环境链 + 按名向上查找 能跑通多数情况，但存在 词法作用域 Bug（外层重新 `var` 后，闭包可能绑错变量）。ch11 引入 语义分析 (Semantic Analysis)：运行前 解析每个变量「在第几层环境」，运行时 按 distance 直取。

---

## 专项笔记

| 阅读 |
|------|
| [11-resolving-and-binding.md](./11-resolving-and-binding.md) |

---

## 逻辑脉络

见 [11-resolving-and-binding.md](./11-resolving-and-binding.md) 内 § 小节与速记。
