# 第 15 章 · A Virtual Machine（虚拟机）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/a-virtual-machine.html) · [中文在线](https://craftinginterpreters.com/a-virtual-machine.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

有了 Chunk（ch14），本章造 执行引擎：栈式 VM + 指令指针 ip + `run()` 主循环。先实现 算术计算器，验证 bytecode 可跑。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §15.1 | 指令执行机（An Instruction Execution Machine） | [01-an-instruction-execution-machine.md](./01-an-instruction-execution-machine.md) |
| §15.2 | 值栈操作（A Value Stack Manipulator） | [02-a-value-stack-manipulator.md](./02-a-value-stack-manipulator.md) |
| §15.3 | 算术计算器（An Arithmetic Calculator） | [03-an-arithmetic-calculator.md](./03-an-arithmetic-calculator.md) |
| ·5 | VM 与 Chunk 协作 | [04-vm-chunk.md](./04-vm-chunk.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
