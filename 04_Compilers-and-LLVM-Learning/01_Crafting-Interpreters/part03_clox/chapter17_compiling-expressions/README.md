# 第 17 章 · Compiling Expressions（编译表达式）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/compiling-expressions.html) · [中文在线](https://craftinginterpreters.com/compiling-expressions.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

前端扫描器（ch16）与后端 VM（ch15）正式接合：编写 字节码编译器，把源码 直接编译成 Chunk，不建 AST。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| ·2 | 单遍编译（Single-Pass Compilation） | [01-single-pass-compilation.md](./01-single-pass-compilation.md) |
| ·3 | 普拉特解析器（A Pratt Parser） | [02-a-pratt-parser.md](./02-a-pratt-parser.md) |
| ·4 | 发出字节码（Emitting Bytecode） | [03-emitting-bytecode.md](./03-emitting-bytecode.md) |
| ·5 | 语法错误处理（Handling Syntax Errors） | [04-handling-syntax-errors.md](./04-handling-syntax-errors.md) |
| ·6 | 编译管线（本章结束时） | [05-compiling-expressions.md](./05-compiling-expressions.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
