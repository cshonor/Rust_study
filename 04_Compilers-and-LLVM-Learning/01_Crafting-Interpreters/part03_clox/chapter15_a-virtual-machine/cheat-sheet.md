# 第 15 章 · A Virtual Machine（虚拟机） · 速记与自测

← [本章目录](./README.md) · 上一节：[04-vm-chunk.md](./04-vm-chunk.md)

---

## 本章速记

```text
§15.1  VM · ip · run() · switch 分发
§15.2  Value Stack · push/pop · STACK_MAX
§15.3  OP_CONSTANT/NEGATE/ADD/.../RETURN · 栈式算术
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **16** | [chapter16 · Scanning](../chapter16_scanning-on-demand/) | **按需** Token · 关键字 DFA |
| **17** | Compiling Expressions | 源码 → **emit Chunk** |
| **18** | Types of Values | **NaN tagging** 统一 Value |

---

---

## 自测

1. `ip` 在读取 `OP_CONSTANT` 的操作数（索引字节）时如何推进？
2. 栈式 VM 执行 `a + b * c` 时，编译器应保证什么顺序？
3. clox 的 `run()` 与 jlox 的 `visitBinaryExpr` 在控制流上有何本质不同？

---

---

## 阅读进度

- [x] §15.1～§15.3 结构梳理（本章笔记）
- [ ] 手写一段 bytecode 用 VM 跑通四则运算
- [ ] 本章 Challenges
