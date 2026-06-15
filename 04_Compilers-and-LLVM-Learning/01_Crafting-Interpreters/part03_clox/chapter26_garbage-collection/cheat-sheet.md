# 第 26 章 · Garbage Collection（垃圾回收） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-gc-clox.md](./05-gc-clox.md)

---

## 本章速记

```text
Mark      根 → 三色/work 栈 → markObject 图遍历
Sweep     链表扫未 mark → free · 清除 mark 位
Strings   intern 表弱引用 · sweep 间清理
Trigger   bytesAllocated vs nextGC · 自适应阈值
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **27** | [chapter27 · Classes](../chapter27_classes-and-instances/) | **`OP_CLASS`** · **`ObjInstance`** |
| **28** | Methods | 方法 · **`this`** |
| **19** ch19 | Strings | 对比 exit-time free → GC |

---

---

## 自测

1. 闭包循环引用两个 Obj，无根可达时 GC 能否回收？
2. 为何 intern 表不能对字符串做普通强 mark？
3. `nextGC` 自适应调大有什么好处与风险？

---

---

## 阅读进度

- [x] Mark-Sweep / 三色 / 弱引用 / 触发 结构梳理（本章笔记）
- [ ] 画一轮 GC 前后 objects 链表与 strings 表
- [ ] 本章 Challenges
