# 第 25 章 · Closures（闭包） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-ast.md](./05-ast.md)

---

## 本章速记

```text
Upvalue     间接层 · ObjClosure · GET/SET_UPVALUE
Flatten     嵌套时中间层传递 upvalue 索引
Close       OP_CLOSE_UPVALUE · open→closed · 防悬空
对照 jlox   Environment+distance ↔ upvalue+slot
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **26** | [chapter26 · GC](../chapter26_garbage-collection/) | **Mark-Sweep** · 三色 |
| **27** | [chapter27 · Classes](../chapter27_classes-and-instances/) | **`ObjClass`** · 实例 |
| **28** | Methods | **`this`** · 方法绑定 |

---

---

## 自测

1. 为何不能 let 闭包长期持有「栈 slot 指针」而不 close？
2. Flattening 时 middle 不捕获 x，inner 会怎样？
3. `ObjClosure` 与 `ObjFunction` 在 `OP_CALL` 路径上的差别？

---

---

## 阅读进度

- [x] Upvalue / Flatten / Close 结构梳理（本章笔记）
- [ ]  trace `outer()` return 后 inner 读 x 的 upvalue 状态
- [ ] 原书 ch24 闭包 Challenges
