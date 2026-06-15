# 第 4 章 · Scanning（扫描 / 词法分析）

> 在线：[scanning.html](https://craftinginterpreters.com/scanning.html) · 中文：[第 4 章 扫描](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch03 Lox 规格](../../part01_welcome/chapter03_the-lox-language/notes/03-the-lox-language.md)

---

## 本章定位

**Part II 第一段代码**：实现 **jlox 扫描器（Scanner）**——编译/解释流水线的**第一站**（[ch02 §2.1.1](../../part01_welcome/chapter02_map-of-the-territory/notes/02-map-of-the-territory.md)）。

| 输入 | 输出 |
|------|------|
| 源代码**字符流** | **Token** 序列（线性） |

下一章 **ch5 Representing Code** 将把 Token 流解析成 **AST**（树状）。

| 小节 | 主题 |
|------|------|
| **§4.1** | jlox 脚手架：`runFile` / REPL / `error()` |
| **§4.2** | Lexeme vs Token |
| **§4.3** | 正则语言、手写扫描循环 |
| **§4.4～4.7** | `Scanner` 类：指针、单/双字符、注释/字符串/数字、保留字 |

---

## §4.1 解释器框架（The Interpreter Framework）

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

## §4.2 词素与词法单元（Lexemes and Tokens）

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

## §4.3 正则语言与表达式（Regular Languages and Expressions）

- 扫描器核心 = **循环**：
  1. 从当前字符识别属于哪个词素
  2. **消耗**这些字符
  3. 生成 Token
  4. 重复直到 EOF

- 工业界常用 **Lex** 等工具：用**正则表达式**自动生成扫描器。
- **本书选择**：**纯手工**实现——弄懂每一行（ch1 §1.2 承诺）。

**理论背景**：词法规则大多属于**正则语言**；比正则更复杂的结构留给 **Parser**（上下文无关文法等，ch6 起）。

---

## §4.4 Scanner 类（The Scanner Class）

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

## §4.5 识别词素（Recognizing Lexemes）

### 单字符词素

- 用 **`switch`** 识别 `( ) { } , . - + ; *` 等 → 直接 `addToken(类型)`。

### 词法错误（Lexical errors）

- 遇到 Lox **不允许**的字符（如 `@`）→ **报错**。
- 关键：**继续扫描**剩余源码 → 一次展示**多个**错误，避免「打地鼠式」修一个报一个。

### 双字符运算符与前瞻（Lookahead）

- `!=`、`==`、`>=` 等需看**下一个**字符。
- **`match(expected)`**：若 `current` 处是 `expected` 则消耗并返回 true。
- 例：见到 `!` → `match('=')` 成功则 `BANG_EQUAL`，否则 `BANG`。

```text
!     → peek ? =  →  !=  vs  !
```

**clox 预告**：Part III **ch16** 同样手写 C 版 Scanner，思路一致。

---

## §4.6 更长的词素（Longer Lexemes）

### 注释与空白

| 情况 | 处理 |
|------|------|
| `//` | 行注释 → 消耗到**行尾** |
| 空格 / `\r` / `\t` | **跳过** |
| `\n` | 跳过并 **`line++`** |

### 字符串字面量

- 从 `"` 开始，读到**闭合 `"`**（Lox 支持**多行**字符串）。
- Token 的 literal：用 **`substring`** 去掉外围引号，得到真实字符串。

### 数字字面量

- Lox 只有 **双精度浮点**（`double` / Java `Double`）。
- 小数点：用 **`peekNext()`** 两步前瞻，确保 `.` 后面还有数字（区分 `123.` 与 `123.45`）。
- 解析：Java 内置字符串 → `Double` 转换。

---

## §4.7 保留字与标识符（Reserved Words and Identifiers）

### 最大吞噬（Maximal munch）

- 多条规则都能匹配时 → 匹配**字符最多**的那条。
- 例：`orchid` 是**标识符**，不会因前缀 `or` 被当成 **`or` 关键字**。

### 实现要点

1. 以**字母或 `_`** 开头 → 进入标识符/关键字路径。
2. 持续消耗后续**字母与数字**。
3. 得到完整字符串 → 查 **保留字 HashMap**（`and`、`class`、`fun`…）。
4. 命中 → **关键字 Token**；未命中 → **IDENTIFIER**。

| 类别 | 示例 Token 类型 |
|------|-----------------|
| 关键字 | `VAR`、`FUN`、`CLASS`、`IF`… |
| 标识符 | `IDENTIFIER` + lexeme 文本 |

---

## 流水线位置与后续章节

```text
ch4 Scanning     → List<Token>
ch5 Representing → AST 节点类
ch6 Parsing      → Token 流 → AST
ch7 Evaluating   → 遍历 AST 求值
```

**clox**：Token 概念相同 · **ch16 Scanning on Demand**（按需产出，非一次性 List）。

---

## 本章速记

```text
§4.1  runFile / REPL / error(line)
§4.2  Lexeme = 文本片段 · Token = 类型 + 字面量 + 行号
§4.4  start / current / line
§4.5  switch 单字符 · 词法错误继续扫 · match 双字符
§4.6  // 注释 · 字符串 · double · peekNext
§4.7  maximal munch · Map 查保留字
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **5** | [chapter05 · Representing Code](../chapter05_representing-code/) | 定义 **AST 节点**（树的数据结构） |
| **6** | Parsing | Token → AST |

写 parser 前务必完成 ch5 的表达式/语句节点类（附录 II 有生成代码参考）。

---

## 自测 / Challenges（可选）

- [ ] 画 `Scanner` 在 `"var x = 1.5; // hi\n"` 上的 `start/current/line` 变化。
- [ ] 列举 3 个需 **lookahead** 的 Lox 词素。
- [ ] 说明 **maximal munch** 若不用，关键字 `or` 与标识符 `orchid` 会如何冲突。
- [ ] 对比 **03**《自制编译器》：JavaCC 生成扫描器 vs 本书手写——各牺牲/得到什么。

---

## 阅读进度

- [x] §4.1～§4.7 结构梳理（本章笔记）
- [ ] 跟书实现 / 对照 GitHub `jlox` 源码
- [ ] 本章 Challenges
