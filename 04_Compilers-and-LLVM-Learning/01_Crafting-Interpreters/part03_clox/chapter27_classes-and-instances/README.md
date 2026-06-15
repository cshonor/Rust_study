# 第 27 章 · Classes and Instances（类与实例）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/classes-and-instances.html) · [中文在线](https://craftinginterpreters.com/classes-and-instances.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

GC（ch26） 就绪后，安全引入 OOP：类是一等堆对象，调用类 = 构造实例；实例用 哈希表 存字段。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| ·2 | 类对象（Class Objects） | [01-class-objects.md](./01-class-objects.md) |
| ·3 | 类的实例化（Instances） | [02-instances.md](./02-instances.md) |
| ·4 | 字段与属性（Get and Set Expressions） | [03-get-and-set-expressions.md](./03-get-and-set-expressions.md) |
| ·5 | 对象模型（clox OOP 栈） | [04-clox-oop.md](./04-clox-oop.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
