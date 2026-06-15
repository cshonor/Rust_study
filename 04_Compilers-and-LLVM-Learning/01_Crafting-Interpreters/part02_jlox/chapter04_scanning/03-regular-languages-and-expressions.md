# 第 4 章 · Scanning（扫描 / 词法分析） · §4.3 正则语言与表达式（Regular Languages and Expressions）

← [本章目录](./README.md) · 上一节：[02-lexemes-and-tokens.md](./02-lexemes-and-tokens.md) · 下一节：[04-the-scanner-class.md](./04-the-scanner-class.md)

---

- 扫描器核心 = **循环**：
  1. 从当前字符识别属于哪个词素
  2. **消耗**这些字符
  3. 生成 Token
  4. 重复直到 EOF

- 工业界常用 **Lex** 等工具：用**正则表达式**自动生成扫描器。
- **本书选择**：**纯手工**实现——弄懂每一行（ch1 §1.2 承诺）。

**理论背景**：词法规则大多属于**正则语言**；比正则更复杂的结构留给 **Parser**（上下文无关文法等，ch6 起）。

---
