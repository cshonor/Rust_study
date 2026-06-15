# 第 4 章 · Scanning（扫描 / 词法分析） · §4.2 词素与词法单元（Lexemes and Tokens）

← [本章目录](./README.md) · 上一节：[01-the-interpreter-framework.md](./01-the-interpreter-framework.md) · 下一节：[03-regular-languages-and-expressions.md](./03-regular-languages-and-expressions.md)

---

| 概念 | 含义 |
|------|------|
| **Lexeme（词素）** | 源码中有独立意义的**最小字符序列**，如 `var`、`language`、`"lox"` |
| **Token（词法单元）** | 扫描器把词素 + 元数据打包成的**对象** |

### Token 对象通常包含

| 字段 | 作用 |
|------|------|
| **Type** | 枚举：关键字、运算符、标点、字面量类别……供 **Parser** 识别 |
| **Literal** | 运行时值：如 `"123"` → `123.0`（`Double`） |
| **Location** | **行号**等，供精确报错 |

```text
源码字符  →  Lexeme（文本片段）  →  Token（类型 + 字面量 + 行号）
```

---
