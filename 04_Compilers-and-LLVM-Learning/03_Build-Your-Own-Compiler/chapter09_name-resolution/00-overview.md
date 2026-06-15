# 第 9 章 · 语义分析（1）引用的消解 · 本章定位

← [本章目录](./README.md) · 上一章：[ch8 抽象语法树的生成](../chapter08_build-ast/README.md) · 下一章：[ch10 语义分析（2）静态类型检查](../chapter10_type-check/)（待建） · 下一节：[01-semantic-overview-visitor.md](./01-semantic-overview-visitor.md)

---

```text
ch8  parse → AST
  ↓
ch9  语义 Pass（1）名字消解  ← 本章
  ↓
ch10 语义 Pass（2）类型检查
  ↓
ch11 AST → IR
```

| [ch2 `compile`](../chapter02_cflat-cbc/03-compiler-control-flow.md) | 本章 |
|-----------------------------------------------------------------------|------|
| parse 后 **语义分析** | AST **不变结构、填符号/link** — 尚未 IR |

**本章完成**：名字 → 定义；类型名 → `Type` 实体 — ch10 才能做表达式/静态类型检查。
