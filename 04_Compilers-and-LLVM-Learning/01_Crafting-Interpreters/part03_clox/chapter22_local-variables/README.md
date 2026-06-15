# 第 22 章 · Local Variables（局部变量）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/local-variables.html) · [中文在线](https://craftinginterpreters.com/local-variables.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

ch21 全局变量走 哈希表按名查找；ch22 引入 块作用域 + 局部变量，变量直接住在 Value Stack 槽位 上，读写 O(1) 按索引。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §22.1 | 表示局部变量（Representing Local Variables） | [01-representing-local-variables.md](./01-representing-local-variables.md) |
| §22.2～ | §22.3 块作用域与声明局部变量 | [02-22-3.md](./02-22-3.md) |
| §22.4 | 使用局部变量（Using Locals） | [03-using-locals.md](./03-using-locals.md) |
| §22.5 | 另一个作用域边缘情况（Another Scope Edge Case） | [04-another-scope-edge-case.md](./04-another-scope-edge-case.md) |
| ·6 | 局部 vs 全局（小结） | [05-local-variables-vs.md](./05-local-variables-vs.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
