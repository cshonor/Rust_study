# 第 16 章 · Scanning on Demand（按需扫描） · 速记与自测

← [本章目录](./README.md) · 上一节：[04-jlox-vs-clox.md](./04-jlox-vs-clox.md)

---

## 本章速记

```text
§16.1–2  scanToken 按需 · Token 按值 · 无全量列表
§16.3     skipWhitespace/注释 · 单/双字符 · lexeme 切片
§16.4     标识符扫描 · switch-DFA 关键字 · 零分配
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **17** | [chapter17 · Compiling Expressions](../chapter17_compiling-expressions/) | Token → **Chunk** · 表达式编译 |
| **18** | Types of Values | **Value** 表示升级 |

---

---

## 自测

1. clox 的 Token 为何只存 `start`+`length` 而不复制字符串？
2. 双字符 `!=` 扫描为什么需要 **lookahead**？
3. 关键字 DFA 与 jlox HashMap 各适合什么语言/runtime 约束？

---

---

## 阅读进度

- [x] §16.1～§16.4 结构梳理（本章笔记）
- [ ] 对照 ch4 jlox：列出 API 差异表
- [ ] 本章 Challenges
