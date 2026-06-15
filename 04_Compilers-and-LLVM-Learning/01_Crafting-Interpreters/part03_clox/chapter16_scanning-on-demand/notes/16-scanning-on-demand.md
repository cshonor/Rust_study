# 第 16 章 · Scanning on Demand（按需扫描）

> 在线：[scanning-on-demand.html](https://craftinginterpreters.com/scanning-on-demand.html) · 中文：[第 16 章 按需扫描](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch15 VM](../chapter15_a-virtual-machine/notes/15-a-virtual-machine.md)

---

## 本章定位

**clox 编译器前端起步**：重写扫描器。与 jlox **一次性扫描成 Token 列表** 不同，clox **按需返回单个 Token**，省内存、与 **递归下降编译器** 天然契合。

| 对比 | jlox ch4 | clox ch16 |
|------|----------|-----------|
| 输出 | **`List<Token>`** 全文件 | **`scanToken()`** 每次 1 个 |
| 存储 | 堆上 Token 对象列表 | Token **按值** 在 C **栈** 上 |
| 调用者 | Parser 迭代列表 | Compiler **`advance()`** 拉下一个 |
| 关键字 | HashMap / 表查找 | **硬编码 DFA / switch 路径** |

| 小节 | 主题 |
|------|------|
| **§16.1～§16.2** | **A Token at a Time** · 按需 API |
| **§16.3** | **Lexeme** · 空白 · 注释 · 双字符 |
| **§16.4** | 标识符 · **关键字 Trie/DFA** |

---

## §16.1～§16.2 按需生成词法单元（A Token at a Time）

**全局扫描状态**（典型）：

| 变量 | 作用 |
|------|------|
| **`scanner.start`** | 当前 lexeme 起始 |
| **`scanner.current`** | 扫描头 |
| **`scanner.line`** | 当前行号（供 ch14 **`lines[]`**） |

**API**：

```c
Token scanToken();   // 跳过空白/注释后，识别下一个 Token
bool  isAtEnd();     // 是否 EOF
```

**编译器侧**：

```c
Token current = advance();   // scanToken + 缓存
Token peek();                // lookahead 不消费
```

| 优点 | 说明 |
|------|------|
| **零 Token 列表** | 大文件也不预分配 |
| **流水线** | Scan → Parse/Compile 交织（Parser 要 Token 才扫） |
| **缓存友好** | 顺序读源码，局部性更好 |

**对照 jlox**：[ch04 Scanning](../../part02_jlox/chapter04_scanning/notes/04-scanning.md) 的 **`Scanner.scanTokens()`** 一次扫完。

---

## §16.3 处理词素与空白（Lexemes and Whitespace）

**扫描循环**（C 手写）：

```text
skipWhitespace():
  空格、制表
  换行 → line++
  // 注释 → 扫到行尾

scanToken():
  skipWhitespace
  start = current
  switch (首字符) { ... }
```

| 类别 | 处理 |
|------|------|
| **单字符** | `( ) { } , . - + ; *` 等 → 直接 Token 类型 |
| **双字符** | **`!` + `=`** → `BANG_EQUAL`；**`=` + `=`** → `EQUAL_EQUAL`；**前瞻 `peek()`** |
| **数字** | 读连续 digit · 可选 `.` 小数部分 → **`NUMBER`** |
| **字符串** | `"..."` 转义处理 → **`STRING`**（完整 lexeme） |

**Token 结构**：

| 字段 | 说明 |
|------|------|
| **`type`** | 枚举 |
| **`start` / `length`** | 指向 **源码缓冲区** 的 lexeme 切片（非拷贝字符串） |
| **`line`** | 行号 |

**Lexeme**：源码中的 **原始字符切片**；关键字与标识符 **词面相同**，靠 §16.4 区分。

---

## §16.4 标识符与关键字（Identifiers and Keywords）

标识符：`[a-zA-Z_][a-zA-Z0-9_]*` → 先扫完整 lexeme。

**关键字识别**：C 无内置 HashMap → 书中用 **确定性有限自动机 (DFA)** / **Trie 路径** = **嵌套 switch**。

```text
首字母 'f':
  后续 "alse"  → TOKEN_FALSE
  后续 "un"    → TOKEN_FUN
  否则         → TOKEN_IDENTIFIER
```

| 特点 | 说明 |
|------|------|
| **无堆分配** | 不建关键字表 |
| **分支预测友好** | 常见字快速路径 |
| **最长匹配** | 整词比对，避免 `for` vs `foreign` 误判 |

**若不是关键字** → **`TOKEN_IDENTIFIER`**（变量名、函数名等）。

**Lox 保留字**（与 ch3 规格一致）：`and class else false fun for if nil or print return super this true var while` 等。

---

## jlox vs clox 扫描器架构

```text
jlox:
  source ──► Scanner.scanTokens() ──► List<Token> ──► Parser

clox:
  source ──► scanToken() ◄── Compiler.advance()（按需）
                │
                └──► 直接 emit bytecode（ch17+）
```

---

## 本章速记

```text
§16.1–2  scanToken 按需 · Token 按值 · 无全量列表
§16.3     skipWhitespace/注释 · 单/双字符 · lexeme 切片
§16.4     标识符扫描 · switch-DFA 关键字 · 零分配
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **17** | [chapter17 · Compiling Expressions](../chapter17_compiling-expressions/) | Token → **Chunk** · 表达式编译 |
| **18** | Types of Values | **Value** 表示升级 |

---

## 自测

1. clox 的 Token 为何只存 `start`+`length` 而不复制字符串？
2. 双字符 `!=` 扫描为什么需要 **lookahead**？
3. 关键字 DFA 与 jlox HashMap 各适合什么语言/runtime 约束？

---

## 阅读进度

- [x] §16.1～§16.4 结构梳理（本章笔记）
- [ ] 对照 ch4 jlox：列出 API 差异表
- [ ] 本章 Challenges
