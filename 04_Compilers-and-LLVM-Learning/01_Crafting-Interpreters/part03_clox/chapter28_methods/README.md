# 第 28 章 · Methods and Initializers（方法与初始化器）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/methods.html) · [中文在线](https://craftinginterpreters.com/methods.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

为 OOP 注入可调用行为：类方法表 · `this` 作 slot 0 · `init` · `OP_INVOKE` 超指令 避免 BoundMethod 堆分配。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §28.1 | 方法声明（Method Declarations） | [01-method-declarations.md](./01-method-declarations.md) |
| §28.2 | 绑定方法（Bound Methods） | [02-bound-methods.md](./02-bound-methods.md) |
| §28.3 | This 关键字（This） | [03-this.md](./03-this.md) |
| §28.4 | 初始化器（Initializers） | [04-initializers.md](./04-initializers.md) |
| ·6 | Superinstruction） | [05-optimized-invocations.md](./05-optimized-invocations.md) |
| ·7 | 方法调用路径（小结） | [06-methods-and-initializers.md](./06-methods-and-initializers.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
