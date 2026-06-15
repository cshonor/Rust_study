# 第 4 章 · 上下文相关分析（Context-Sensitive Analysis）

> **Engineering a Compiler 3e** · [02 编译器工程](../../README.md) · [本书目录](../../本书目录.md) · Part I 前端

## 状态

- [x] 已读（笔记整理）

---

## 一句话

ch2/ch3 只证明「**句子结构**合法」；**ch4 证明「含义**在上下文中**合法**」— 声明先于使用、类型兼容、数组维数匹配等。核心工具：**类型系统**、**属性文法（理论）**、**语法制导翻译（工程）** → 构建 **AST**、符号表，为 **IR** 扫清语义障碍。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 为何需要上下文相关分析 | [01-why-context-sensitive.md](./01-why-context-sensitive.md) |
| §2 | 类型系统 | [02-type-systems.md](./02-type-systems.md) |
| §3 | 属性文法 | [03-attribute-grammars.md](./03-attribute-grammars.md) |
| §4 | 语法制导翻译（工程实践） | [04-syntax-directed-translation.md](./04-syntax-directed-translation.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch4 | 对照 |
|----------|------|
| 语义 / 作用域 | [CI jlox ch11 Resolver](../../../01_Crafting-Interpreters/part02_jlox/chapter11_resolving-and-binding/README.md) |
| 类型（动态） | [CI jlox ch7 Evaluating](../../../01_Crafting-Interpreters/part02_jlox/chapter07_evaluating-expressions/README.md) |
| AST | [CI jlox ch5 Representing Code](../../../01_Crafting-Interpreters/part02_jlox/chapter05_representing-code/README.md) |
| Rust 静态类型 | RFR [第 2 章类型](../../02-RFR/Chapter-02-Types/) · `rustc` 借用检查 |
| ch3 语法 | [ch3 语法分析](../chapter03_parsers/README.md) |

---

## 逻辑脉络

CFG 不够 → 语义分析 / 类型检查 → 属性文法（理论）→ 嵌入 Action 的 SDT（工程）→ AST + 符号表。
