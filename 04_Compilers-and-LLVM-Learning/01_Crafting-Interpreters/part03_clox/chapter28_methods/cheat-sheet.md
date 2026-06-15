# 第 28 章 · Methods and Initializers（方法与初始化器） · 速记与自测

← [本章目录](./README.md) · 上一节：[06-chunks-of-bytecode-chunk.md](./06-chunks-of-bytecode-chunk.md)

---

## 本章速记

```text
§28.1  class.methods Table · ObjClosure
§28.2  ObjBoundMethod · receiver+method
§28.3  this = slot 0 · GET/SET_LOCAL 0
§28.4  init 自动调用 · 只 return this
§28.5  OP_INVOKE 融合查+调 · 免堆分配
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **29** | [chapter29 · Superclasses](../chapter29_wrappers-and-api/README.md) | **`OP_INHERIT`** · **`super`** |
| **30** | [chapter30 · Optimization](../chapter30_optimization/) | 掩码 · **NaN boxing** |

---

---

## 自测

1. 为何 `this` 用 slot 0 而不是单独 `OP_THIS`？
2. `OP_INVOKE` 相对 Get+Call 省掉了哪次堆分配？
3. 实例字段与方法同名时，`OP_INVOKE` 应命中哪一个？

---

---

## 阅读进度

- [x] §28.1～§28.5 结构梳理（本章笔记）
- [ ] 对照 benchmark 理解 invoke 提升
- [ ] 本章 Challenges
