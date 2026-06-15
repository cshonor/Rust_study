# 第 24 章 · Calling and Closures（调用与函数）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/calling-and-closures.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

此前 clox 仅在 顶层脚本 运行；本章起支持 `fun`、调用、递归、return、原生函数。函数体有 独立 Chunk + CallFrame；参数与局部变量共享 同一块栈窗口（零拷贝传递）。

---

## 专项笔记

| 阅读 |
|------|
| [24-calling-and-closures.md](./24-calling-and-closures.md) |

---

## 逻辑脉络

见 [24-calling-and-closures.md](./24-calling-and-closures.md) 内 § 小节与速记。
