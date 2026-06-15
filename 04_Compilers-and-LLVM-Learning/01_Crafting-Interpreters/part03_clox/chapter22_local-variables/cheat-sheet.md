# 第 22 章 · Local Variables（局部变量） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-ast.md](./05-ast.md)

---

## 本章速记

```text
§22.1  locals[] · scopeDepth · slot = 栈槽
§22.2–3 begin/endScope · 从后向前 resolve · shadowing
§22.4   OP_GET/SET_LOCAL 单字节索引
§22.5   var a=a · 未初始化 vs markInitialized
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **23** | [chapter23 · Jumping](../chapter23_jumping-back-and-forth/) | **回填** · `OP_JUMP` · 控制流 |
| **24** | Calling and Closures | **CallFrame** · **`OP_CALL`** |
| **11** jlox | Resolver | distance ↔ clox slot/upvalue |

---

---

## 自测

1. 为什么局部变量不需要放进常量池里的名字？
2. `endScope` 时编译器丢弃 locals，运行时栈如何同步变短？
3. `var a = a` 若不做两阶段，内层 `a` 解析会错成什么？

---

---

## 阅读进度

- [x] §22.1～§22.5 结构梳理（本章笔记）
- [ ] 手工 trace 嵌套 `{}` 的 locals 与 scopeDepth
- [ ] 本章 Challenges
