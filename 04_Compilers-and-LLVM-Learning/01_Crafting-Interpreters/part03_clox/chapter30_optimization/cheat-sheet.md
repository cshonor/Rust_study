# 第 30 章 · Optimization（优化） · 速记与自测

← [本章目录](./README.md) · 上一节：[04-optimization-30.md](./04-optimization-30.md)

---

## 本章速记

```text
§30.1–2  cap=2^n → index = hash & (cap-1) · ~2× on hash-heavy
§30.3     NaN box · 64-bit Value · ptr/bool/nil in NaN space
方法      benchmark → profile → peephole（ch28 invoke 同理）
```

---

---

## 读后延伸

| 方向 | 链接 |
|------|------|
| **LLVM 优化** | [04_Learn-LLVM-17](../../../04_Learn-LLVM-17/) |
| **编译器 SSA** | [02 编译器工程](../../../02_Compiler-Principles/) |
| **附录** | [Appendix I Lox 语法](../../backmatter/appendix01_lox-grammar/) |
| **Wrappers** | [ch29 原书 REPL/API](https://craftinginterpreters.com/wrappers-and-api.html) |

---

---

## 自测

1. 为何 `& (capacity-1)` 要求 capacity 是 2 的幂？
2. NaN boxing 后 `IS_NUMBER` 如何判断一个 `uint64_t`？
3. 若不做 NaN box，栈 256 深时多浪费多少字节（相对 8B slot）？

---

---

## 阅读进度

- [x] §30.1～§30.3 结构梳理（本章笔记）
- [x] **Crafting Interpreters 正文 30 章笔记链路贯通**
- [ ] 跑书中 benchmark · 对比 mask / NaN 前后
- [ ] 本章 Challenges · 全书 Challenges 复盘
