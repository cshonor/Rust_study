# 第 24 章 · Calling and Closures（调用与函数） · 速记与自测

← [本章目录](./README.md) · 上一节：[07-call.md](./07-call.md)

---

## 本章速记

```text
§24.1–2 ObjFunction · chunk · arity
§24.3   CallFrame[] · ip + slots 每帧独立
§24.4–5 OP_CALL · 参数槽与 locals 窗口重叠
§24.6   OP_RETURN 弹帧 · 返回值压回 caller
§24.7   ObjNative · clock()
后续    [ch25 Closures](../chapter25_objects/README.md)
```

---

---

## 读后下一步

| 章 / 节 | 内容 |
|---------|------|
| **ch24 续** | [ch25 · Closures](../chapter25_objects/README.md) | **Upvalue** · **`OP_CLOSE_UPVALUE`** |
| **25** | [chapter25 · Objects](../chapter25_objects/) | 实例字段 |
| **26** | Garbage Collection | **mark-sweep** |

---

---

## 自测

1. 为什么 `CallFrame.slots` 指向 callee 而不是 arg1？
2. 递归时 `frameCount` 与 Java 调用栈的对应关系？
3. Native 与 Function 在 `OP_CALL` 里分歧在哪一步？

---

---

## 阅读进度

- [x] §24.1～§24.7 结构梳理（本章笔记）
- [ ] 画三次递归 `fact(n)` 的 frames 与 stack
- [ ] 继续阅读同章 Upvalue / 闭包小节
- [ ] 本章 Challenges
