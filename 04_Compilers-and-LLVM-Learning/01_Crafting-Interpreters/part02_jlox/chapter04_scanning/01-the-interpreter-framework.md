# 第 4 章 · Scanning（扫描 / 词法分析） · §4.1 解释器框架（The Interpreter Framework）

← [本章目录](./README.md) · 上一节：[00-overview.md](./00-overview.md) · 下一节：[02-lexemes-and-tokens.md](./02-lexemes-and-tokens.md)

---

在写 Scanner 之前，先搭 **jlox  Java 脚手架**：

| 入口 | 行为 |
|------|------|
| **`runFile(path)`** | 读入**整个文件** → 执行 |
| **`runPrompt()`** | **REPL**（Read-Eval-Print Loop）交互 |

### 错误处理

- 基础 **`error(line, message)`**：报告语法/词法错误**行号**。
- 原则：遇到已知错误时**不要**继续执行残缺代码——良好错误处理是**语言设计**的一环（与 ch1 Design Notes 呼应）。

**本仓库联想**：`rustc` 结构化诊断 · RFR 第 4 章错误处理 · 写 Lox 时先保证「报错行号准」。

---
