# 第 2 章 · A Map of the Territory（领域地图） · §2.1 语言的组成部分（The Parts of a Language）

← [本章目录](./README.md) · 上一节：[00-overview.md](./00-overview.md) · 下一节：[02-shortcuts-and-alternate-routes.md](./02-shortcuts-and-alternate-routes.md)

---

完整 **Pipeline**（那座「山」上的途径点）：

```text
Source Code（源码）
    ↑ 上山 · 前端
    Scanning / Lexing     → Tokens
    Parsing               → AST
    （语义分析 / 优化等）  → 常在此后进入 IR
    Intermediate Rep (IR) → 便于优化与后端
    ↓ 下山 · 后端
    Code Generation       → 机器码或字节码
    Virtual Machine       → 可选：跨平台执行字节码
    Runtime               → 类型检查、GC、内置服务等
    Machine Code / 运行结果
```

### §2.1.1 扫描 / 词法分析（Scanning / Lexing）

| 项目 | 说明 |
|------|------|
| **位置** | 流水线**第一步** |
| **输入** | 源代码**字符流** |
| **输出** | **Token**（词法单元）序列 |
| **核心动作** | 逐字符读入 → 识别**词素（Lexeme）** → 打包成带类型的 Token |
| **丢弃** | 空白、换行、注释（不进入 Token 流） |

**Lexeme vs Token**

| | **Lexeme** | **Token** |
|---|------------|-----------|
| 是什么 | 源码里的**文本片段** | 扫描器产出的**结构化对象** |
| 例子 | 字符 `123` | `NUMBER(123.0)` + 行号 |
| 谁用 | 人眼在源码里看到 | **Parser** 读 Token 流建 AST |

```text
源码字符  →  Lexeme（文本）  →  Token（类型 + 字面量 + 位置）
```

---

#### 例子 1 · Lox 一行语句（本书语言）

**源码：**

```lox
var price = 19.99; // unit price
```

**扫描后 Token 流**（空白与注释已丢弃）：

```text
VAR          "var"        @1
IDENTIFIER   "price"      @1
EQUAL        "="          @1
NUMBER       19.99        @1
SEMICOLON    ";"          @1
EOF                       @2
```

**逐字符怎么走**（简化）：

```text
v a r   → 关键字 VAR
p r i c e → IDENTIFIER（保留字表里没有 price）
=       → EQUAL
1 9 . 9 9 → NUMBER（读到非数字/非小数点停止）
;       → SEMICOLON
/ / ... → 整行注释，跳过，不产生 Token
```

**多字符运算符**（不能拆成两个 Token）：

```lox
a != b
```

```text
IDENTIFIER "a"
BANG_EQUAL "!="    ← 一个 Token，不是 BANG + EQUAL
IDENTIFIER "b"
```

→ 实现细节：[jlox ch4 · 识别词素](../../part02_jlox/chapter04_scanning/05-recognizing-lexemes.md) · [保留字与标识符](../../part02_jlox/chapter04_scanning/07-reserved-words-and-identifiers.md)

---

#### 例子 2 · Rust 一行（对照 rustc 前端）

**源码：**

```rust
let x: u64 = 10; // counter
```

**等价 Token 流（概念上，与 `rustc` 内部一致）：**

```text
KW_LET       "let"
IDENT        "x"
COLON        ":"
IDENT        "u64"        ← 类型名在词法阶段只是标识符
EQ           "="
LIT_INT      10           ← 字面量已带数值/类型信息
SEMI         ";"
EOF
```

**与 Lox 的相同点**：都是**线性 Token 列表**，Parser 下一步才关心 `let x: u64 = 10` 的嵌套结构。  
**不同点**：Rust 词法更复杂（生命周期 `'a`、raw string `r#"..."#`、字节串 `b"..."` 等）。

---

#### 例子 3 · 字符串与数字（词素边界）

**Lox：**

```lox
print "hi\n";
```

| 阶段 | 内容 |
|------|------|
| 源码字符 | `"` `h` `i` `\` `n` `"` `;` |
| Lexeme | `"hi\n"`（含转义，引号不算进字面量） |
| Token | `STRING` · literal = 运行时字符串 `hi` + 换行 · `SEMICOLON` |

**Rust 整数后缀：**

```rust
let n = 0xff_u32;
```

```text
LIT_INT 255 (type: u32)   ← 词法阶段把 0xff 与 _u32 后缀一起读成带类型的字面量 Token
SEMI
```

---

#### 例子 4 · 什么不会变成 Token

| 源码片段 | 扫描器行为 |
|----------|------------|
| `   ` 空格 / `\t` | 跳过 |
| `// comment` | 跳过到行尾 |
| `/* block */` | 跳过（Rust/Lox 规则略不同） |
| `#![allow(...)]` | Rust **属性**，仍由词法/后续阶段处理，不是普通注释 |

Parser **永远看不到**注释文本 — 报错行号仍可能指向原文件第 N 行（Token 上常带 `line` 字段）。

---

#### 例子 5 · 易错边界（自测用）

**1. `=` vs `==`**

```lox
a = 1;   // EQUAL
a == 1;  // EQUAL_EQUAL（多读一个 =）
```

**2. 标识符 vs 关键字**

```lox
var var = 1;   // VAR · IDENTIFIER("var") · EQUAL · NUMBER · SEMICOLON
               // 第二个 var 是变量名，不是关键字
```

**3. 数字紧贴标识符**

```lox
foo123   // 一个 IDENTIFIER "foo123"，不是 foo + 123
123foo   // 通常词法错误：数字后不能直接接字母
```

---

**一句话**：扫描器只做「**切蛋糕**」— 把字符流切成 Token 串；**不管**语法树、不管类型、不管 `=` 两边是否类型匹配（那是 Parser / 语义分析的事）。

**本书对应**：Part II **ch4 Scanning**（jlox）· Part III **ch16 Scanning on Demand**（clox）。

**本仓库**：**03**《自制编译器》词法 · `00-Book` 宏/token 概念 · 深入 [ch4 §4.2 Lexeme 与 Token](../../part02_jlox/chapter04_scanning/02-lexemes-and-tokens.md)

---
### §2.1.2 解析（Parsing）

| 项目 | 说明 |
|------|------|
| **作用** | 赋予代码**语法结构**（像句子成分分析） |
| **输入** | 扁平 **Token 序列** |
| **输出** | **AST**（抽象语法树），反映嵌套层级 |

**本书对应**：Part II **ch5～6**（表示代码 + 解析表达式）起建 jlox 的 AST。

---

### 中间表示（Intermediate Representations, IR）

- 将前端产物转为**更便于后续处理**的中间数据结构。
- 常见分工：
  - **前端 IR**：贴近源语言（AST 可视为一种 IR）。
  - **后端 IR**：贴近目标机器；便于**优化**与**代码生成**。

**本书对应**：

- jlox：以 **AST** 为主，不单独建工业级 IR。
- clox：**字节码 chunk** 即本书的 IR 形态。
- **04 Learn LLVM 17**：Rust → **LLVM IR**（SSA）· 对照 RFR 第 2 章分发 · ch07 优化。

**本仓库联想**：RFR [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md) · [03-2 OS/LLVM 内存布局](../../../../02-RFR/Chapter-01-Foundations/03-2-os-memory-layout.md) 里的 `alloca` / heap。

---

### 代码生成（Code Generation）

- 进入后端 **「下山」**：表示越来越**原始**。
- 产出 CPU（或 VM）能执行的**底层指令**。

**路径分支**：

| 目标 | 结果 |
|------|------|
| 直接生成**机器码** | 快，但绑定具体芯片 |
| 生成**字节码** + VM | 可移植（见 §2.1.7） |

**本书对应**：Part III clox 从 **ch17 Compiling Expressions** 起把 AST/语法编译为字节码。

---

### §2.1.7 虚拟机（Virtual Machine）

- 若只生成**某一芯片**的机器码 → 语言**丧失跨平台**。
- 常见解法：针对**假想芯片**设计**便携式指令** → **字节码（Bytecode）**。
- 用**软件模拟**的执行引擎 = **虚拟机**。

**本书对应**：Part III 全程 **`clox`** = 字节码 + VM（ch14～15 起）。

**本仓库联想**：JVM / WASM / Python VM；RFR 第 8 章 async 状态机与「解释执行循环」可类比 VM 主循环。

---

### §2.1.8 运行时（Runtime）

- 程序**真正跑起来**后的支撑层。
- 职责示例：
  - 动态**类型检查**
  - **垃圾回收（GC）** 等内存服务
  - 内置函数、模块加载等

**本书对应**：jlox 借 **Java GC**；clox **ch26 Garbage Collection** 自实现 GC。

**本仓库**：RFR 第 1 章内存区域 · 第 9～10 章 unsafe/并发 · **Nomicon** 布局与有效性。

→ **Rust/HFT 分层对照**（上山前端 + 下山后端 · `no_std` · `dyn` · FFI）：[04-rust-hft-编译流水线对照.md](./04-rust-hft-编译流水线对照.md)

---
