# 第 25 章 · Closures（闭包）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/calling-and-closures.html) · [中文在线](https://craftinginterpreters.com/calling-and-closures.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

jlox 闭包靠 Java GC + Environment 捕获；clox 局部变量在 栈槽 上，函数返回即 弹栈销毁 → 闭包若仍指向栈地址 = 悬空指针。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| ·2 | Upvalue（上值） | [01-closures-upvalue.md](./01-closures-upvalue.md) |
| ·3 | 扁平化 Upvalues（Flattening Upvalues） | [02-flattening-upvalues.md](./02-flattening-upvalues.md) |
| ·4 | 闭合 Upvalue（Closing Upvalues） | [03-closing-upvalues.md](./03-closing-upvalues.md) |
| ·5 | ObjClosure 与 OP_CLOSURE | [04-objclosure-op-closure.md](./04-objclosure-op-closure.md) |
| ·6 | 闭包生命周期（总览） | [05-closures.md](./05-closures.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
