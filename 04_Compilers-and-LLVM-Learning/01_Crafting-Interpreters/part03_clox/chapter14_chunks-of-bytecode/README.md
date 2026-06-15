# 第 14 章 · Chunks of Bytecode（字节码块）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/chunks-of-bytecode.html) · [中文在线](https://craftinginterpreters.com/chunks-of-bytecode.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

Part III 奠基：在造 VM 之前，先定义 字节码如何存放。clox 用 C 手写 紧凑指令序列 + 常量池 + 行号表，告别 jlox 的 AST 指针树。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §14.1 | 什么是字节码（What is Bytecode?） | [01-what-is-bytecode.md](./01-what-is-bytecode.md) |
| §14.3 | 指令块（Chunks of Instructions） | [02-chunks-of-instructions.md](./02-chunks-of-instructions.md) |
| §14.4 | 反汇编块（Disassembling Chunks） | [03-disassembling-chunks.md](./03-disassembling-chunks.md) |
| §14.5 | 常量（Constants） | [04-constants.md](./04-constants.md) |
| §14.6 | 行号信息（Line Information） | [05-line-information.md](./05-line-information.md) |
| ·7 | Chunk 内存布局（小结） | [06-chunks-of-bytecode-chunk.md](./06-chunks-of-bytecode-chunk.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
