# 第 19 章 · Strings（字符串）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/strings.html) · [中文在线](https://craftinginterpreters.com/strings.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

bool / number / nil 可 按值 塞进 `Value`；字符串长度可变、堆分配 → 引入 Obj 对象系统，为 GC（ch26）与哈希表键（ch20）铺路。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| ·2 | 对象与结构体继承（Values and Objects） | [01-values-and-objects.md](./01-values-and-objects.md) |
| ·3 | 内存管理基础（Memory Management） | [02-memory-management.md](./02-memory-management.md) |
| ·4 | ObjString 要点 | [03-objstring.md](./03-objstring.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
