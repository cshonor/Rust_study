# 第 25 章 · Closures（闭包）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/calling-and-closures.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

jlox 闭包靠 Java GC + Environment 捕获；clox 局部变量在 栈槽 上，函数返回即 弹栈销毁 → 闭包若仍指向栈地址 = 悬空指针。

---

## 专项笔记

| 阅读 |
|------|
| [25-closures.md](./25-closures.md) |

---

## 逻辑脉络

见 [25-closures.md](./25-closures.md) 内 § 小节与速记。
