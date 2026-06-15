# 第 23 章 · Jumping Back and Forth（来回跳转） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-ast.md](./05-ast.md)

---

## 本章速记

```text
§23.1  if · JUMP_IF_FALSE/JUMP · backpatch 占位再修补
§23.2  and/or 短路 · 栈顶留左值 · 复用条件跳转
§23.3  while · OP_LOOP 回跳
§23.4  for 脱糖为 init+while+increment
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **24** | [chapter24 · Calls](../chapter24_calling-and-closures/) | **CallFrame** · **`OP_CALL`** · return |
| **25** | Objects | 堆对象扩展 |

---

---

## 自测

1. 为什么回填偏移用「字节」而不是「指令序号」？
2. `a or b` 在左 truthy 时栈上最终剩几个值？
3. `for` 脱糖后，`continue`（若实现）应改哪几个 jump 目标？

---

---

## 阅读进度

- [x] §23.1～§23.4 结构梳理（本章笔记）
- [ ] 画一条 `if/else` 的 bytecode 布局与 patch 点
- [ ] 本章 Challenges
