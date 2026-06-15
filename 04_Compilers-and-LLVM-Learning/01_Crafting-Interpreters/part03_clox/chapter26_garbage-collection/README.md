# 第 26 章 · Garbage Collection（垃圾回收）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/garbage-collection.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

ch19 起 堆分配（字符串、函数、闭包、Upvalue…）仅靠 进程退出 `freeObjects` 不够 → 手写 Mark-Sweep GC，替换 ch19 的朴素释放。

---

## 专项笔记

| 阅读 |
|------|
| [26-garbage-collection.md](./26-garbage-collection.md) |

---

## 逻辑脉络

见 [26-garbage-collection.md](./26-garbage-collection.md) 内 § 小节与速记。
