# 第 20 章 · Hash Tables（哈希表）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/hash-tables.html) · [中文在线](https://craftinginterpreters.com/hash-tables.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

动态语言的核心基础设施：全局变量、属性、方法表 都依赖 O(1) 均摊 查找。clox 手写 C 哈希表，并为 字符串驻留 (interning) 服务。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| ·2 | 开放寻址与线性探测（Open Addressing and Linear Probing） | [01-open-addressing-and-linear-probing.md](./01-open-addressing-and-linear-probing.md) |
| ·3 | 哈希函数与负载因子（Hash Functions and Load Factor） | [02-hash-functions-and-load-factor.md](./02-hash-functions-and-load-factor.md) |
| ·4 | 字符串驻留（String Interning） | [03-string-interning.md](./03-string-interning.md) |
| ·5 | Table API（概念） | [04-hash-tables-table-api.md](./04-hash-tables-table-api.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
