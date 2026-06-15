# 第 4 章 · Scanning（扫描 / 词法分析） · §4.4 Scanner 类（The Scanner Class）

← [本章目录](./README.md) · 上一节：[03-regular-languages-and-expressions.md](./03-regular-languages-and-expressions.md) · 下一节：[05-recognizing-lexemes.md](./05-recognizing-lexemes.md)

---

- 输入：源代码 **String**
- 输出：`List<Token>`（或按需生成，clox ch16 再议）

### 三个整型指针（偏移量）

| 指针 | 含义 |
|------|------|
| **`start`** | 当前正在扫描的**词素**的第一个字符 |
| **`current`** | 当前**正在检查**的字符 |
| **`line`** | 当前**行号**（报错用） |

典型流程：`start` 标记词素起点 → 不断 `advance()` 移动 `current` → 词素结束 → `addToken(...)` → 更新 `start`。

---
