# 第 16 章 · Scanning on Demand（按需扫描）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/scanning-on-demand.html) · [中文在线](https://craftinginterpreters.com/scanning-on-demand.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

clox 编译器前端起步：重写扫描器。与 jlox 一次性扫描成 Token 列表 不同，clox 按需返回单个 Token，省内存、与 递归下降编译器 天然契合。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §16.1～ | §16.2 按需生成词法单元（A Token at a Time） | [01-a-token-at-a-time.md](./01-a-token-at-a-time.md) |
| §16.3 | 处理词素与空白（Lexemes and Whitespace） | [02-lexemes-and-whitespace.md](./02-lexemes-and-whitespace.md) |
| §16.4 | 标识符与关键字（Identifiers and Keywords） | [03-identifiers-and-keywords.md](./03-identifiers-and-keywords.md) |
| ·5 | jlox vs clox 扫描器架构 | [04-jlox-vs-clox.md](./04-jlox-vs-clox.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
