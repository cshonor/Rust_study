# 第 26 章 · Garbage Collection（垃圾回收）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/garbage-collection.html) · [中文在线](https://craftinginterpreters.com/garbage-collection.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

ch19 起 堆分配（字符串、函数、闭包、Upvalue…）仅靠 进程退出 `freeObjects` 不够 → 手写 Mark-Sweep GC，替换 ch19 的朴素释放。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| ·2 | 标记阶段与三色抽象（Marking & Tri-color Abstraction） | [01-marking-tri-color-abstraction.md](./01-marking-tri-color-abstraction.md) |
| ·3 | 清除阶段（Sweeping） | [02-sweeping.md](./02-sweeping.md) |
| ·4 | 弱引用与字符串池（Weak References） | [03-weak-references.md](./03-weak-references.md) |
| ·5 | 触发回收（When to Collect） | [04-when-to-collect.md](./04-when-to-collect.md) |
| ·6 | GC 与 clox 子系统 | [05-gc-clox.md](./05-gc-clox.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
