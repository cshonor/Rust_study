# 第 12 章 · Classes（类） · 速记与自测

← [本章目录](./README.md) · 上一节：[07-ch12-vs-ch13.md](./07-ch12-vs-ch13.md)

---

## 本章速记

```text
§12.1  Stmt.Class · LoxClass
§12.2  Breakfast() 无 new · LoxInstance
§12.3  Get/Set · instance.fields Map
§12.4  字段优先 · LoxBoundMethod
§12.5  this 环境 · Resolver 限制
§12.6  init 自动调用 · 禁 return 值
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **13** | [chapter13 · Inheritance](../chapter13_inheritance/) | **`super`** · 方法继承 |
| **14+** | [part03_clox](../../part03_clox/) | 字节码 VM 重新开始 |

---

---

## 自测

1. 实例字段与方法同名时，`obj.name` 解析到哪？
2. 为什么 `init` 要单独限制 `return`？
3. `LoxBoundMethod` 和裸 `LoxFunction` 在调用路径上差在哪一步？

---

---

## 阅读进度

- [x] §12.1～§12.6 结构梳理（本章笔记）
- [ ] 画 `bacon.cook()` 的 Get → Call → this 环境链
- [ ] 本章 Challenges
