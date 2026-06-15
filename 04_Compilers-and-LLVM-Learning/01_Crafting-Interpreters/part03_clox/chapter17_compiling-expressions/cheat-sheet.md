# 第 17 章 · Compiling Expressions（编译表达式） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-ast.md](./05-ast.md)

---

## 本章速记

```text
单遍    无 AST · parse 同时 emit Chunk
Pratt   rules[] · prefix/infix · precedence
Emit    先 compile 子表达式再 OP_* · 一元 − 后 NEGATE
错误    panicMode + synchronize 到语句边界
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **18** | [chapter18 · Types of Values](../chapter18_types-of-values/) | **`Value`**  tagged union · bool/nil |
| **19** | Strings | **`ObjString`** · 堆对象 |
| **21** | Global Variables | **`canAssign`** · 全局变量 opcode |

---

---

## 自测

1. Pratt 与 jlox 递归下降相比，换运算符优先级时改哪里？
2. 为什么 `-123` 的编译顺序是 `CONSTANT 123` 然后 `OP_NEGATE`？
3. `panicMode` 存在的意义是什么——若没有会怎样？

---

---

## 阅读进度

- [x] 单遍 / Pratt / Emit / 错误恢复 结构梳理（本章笔记）
- [ ] 对照 `rules[]` 列出各 Token 的 prefix/infix
- [ ] 本章 Challenges
