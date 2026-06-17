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
| **位置** | 流水线**第二步**（接在 Scanning 之后） |
| **输入** | 扁平 **Token 序列** |
| **输出** | **AST**（抽象语法树），反映嵌套层级 |
| **核心动作** | 按**语法规则（文法）**消费 Token → 建树 |
| **不做** | 类型检查、作用域绑定、优化（属语义分析） |

**Token 流 vs AST**

| | **Token 流** | **AST** |
|---|--------------|---------|
| 形状 | **线性**列表 | **树**（父节点 = 语法结构，子节点 = 组成部分） |
| 信息 | 只有「下一个是什么符号」 | 表达「谁包含谁、谁先算谁」 |
| 谁产出 | Scanner（§2.1.1） | **Parser** |
| 谁消费 | Parser | 解释器 / 编译器后端 / 语义分析 |

```text
Token 流（扁平）  →  Parser（按文法匹配）  →  AST（树）
```

---

#### 例子 1 · Lox 变量声明（Token → 语句树）

**接 §2.1.1 的 Token 流：**

```text
VAR · IDENTIFIER("price") · EQUAL · NUMBER(19.99) · SEMICOLON
```

**Parser 产出的 AST（概念结构）：**

```text
Stmt.Var
├── name: "price"
└── initializer: Expr.Literal(19.99)
```

**对比**：Token 流是 5 个平铺符号；AST 说清「这是一条 `var` 声明，名字叫 price，初值是数字 19.99」。

---

#### 例子 2 · 表达式优先级（Parser 的核心价值）

**源码：**

```lox
1 + 2 * 3
```

**Token 流（7 个）：**

```text
NUMBER(1) · PLUS · NUMBER(2) · STAR · NUMBER(3) · EOF
```

**错误理解**（若按 Token 顺序线性读）：「从左到右依次加/乘」→ 会得到 `(1+2)*3 = 9`。

**正确 AST**（Parser 按优先级建树）：

```text
        Binary(+)
       /         \
  Literal(1)   Binary(*)
              /         \
        Literal(2)   Literal(3)
```

**AstPrinter 风格输出**（jlox ch5）：

```text
(+ 1 (* 2 3))    →  先算 2*3，再加 1  →  7
```

**规则**：优先级高的运算符在树上**更深**（更靠近叶子）。Lox 分层（低→高）：`equality` → `comparison` → `term(+/-)` → `factor(*/)` → `unary` → `primary`。

→ [ch6 §6.1 歧义与优先级](../../part02_jlox/chapter06_parsing-expressions/01-ambiguity-and-the-parsing-game.md)

---

#### 例子 3 · 左结合（同级运算符）

**源码：**

```lox
10 - 3 - 1
```

**AST（左结合）：**

```text
      Binary(-)
     /         \
 Binary(-)   Literal(1)
 /         \
Lit(10)   Lit(3)

→ ((10 - 3) - 1) = 6
```

Parser 典型写法（递归下降）：`while (match(MINUS))` 不断把左子「长高」→ 自然左结合。

→ [ch6 §6.2 递归下降](../../part02_jlox/chapter06_parsing-expressions/02-recursive-descent-parsing.md)

---

#### 例子 4 · Rust 对照（`let` + 类型 + 块）

**源码：**

```rust
let x: u64 = 10;
```

**Token 流（概念）：**

```text
KW_LET · IDENT("x") · COLON · IDENT("u64") · EQ · LIT_INT(10) · SEMI
```

**AST 片段（概念，类似 `rustc` HIR 之前的语法树）：**

```text
StmtLet
├── pat: PatIdent("x")
├── ty:  TyPath("u64")      ← Parser 只认「类型语法形状」
└── init: ExprLit(10)
```

**Parser 仍不管**：`u64` 是否已定义、`10` 是否溢出 — 那是**语义分析 / 类型检查**（Rust 的 borrowck 更在后面）。

---

#### 例子 5 · 分组改变结构

**源码：**

```lox
(1 + 2) * 3
```

**Token 流：**

```text
LEFT_PAREN · NUMBER(1) · PLUS · NUMBER(2) · RIGHT_PAREN · STAR · NUMBER(3)
```

**AST：**

```text
      Binary(*)
     /         \
 Binary(+)   Literal(3)
 /       \
Lit(1)  Lit(2)

→ (1+2)*3 = 9   （与 例子 2 的 1+2*3 不同）
```

括号在 Parser 里通常由 **`primary` 规则**处理：遇到 `(` → 递归调用 `expression()` → 期望 `)`。

---

#### 例子 6 · Parser 报什么错（词法过关、语法不过）

| 源码 | Scanner | Parser |
|------|---------|--------|
| `var x = ;` | ✅ Token 正常 | ❌ `=` 后期望表达式，遇到 `;` |
| `if (cond) {` 缺 `}` | ✅ | ❌ 块未闭合 |
| `(1 + 2` | ✅ | ❌ 缺 `)` |
| `@foo` | ❌ 非法字符（扫描阶段） | — |

Parser 用 **`match` / `check` / `advance`** 同步 Token 流；期望的类型对不上 → **语法错误**（jlox §6.3 精确定位行号）。

→ [ch6 §6.3 语法错误](../../part02_jlox/chapter06_parsing-expressions/03-syntax-errors.md)

---

#### 例子 7 · 递归下降在脑中的调用栈

**源码：** `1 + 2 * 3`

```text
expression()
  └─ equality()
       └─ comparison()
            └─ term()          ← 看到 1 + …
                 ├─ factor() → Literal(1)
                 ├─ match(PLUS) ✓
                 └─ term 循环结束，但 + 右操作数要更高优先级层：
                      factor() → 进入 term 内再调 factor
                           ├─ factor() → Literal(2)
                           ├─ match(STAR) ✓
                           └─ factor() → Literal(3)
                 → 组装 Binary(+, 1, Binary(*, 2, 3))
```

**一句话**：低优先级函数包外层，内部调用高优先级函数 → 树自然正确。

---

#### 例子 8 · 易错边界（自测用）

**1. Parser 不检查类型**

```lox
var x = "hello" + 3;   // AST 合法；运行时才可能报错
```

**2. 同一串 Token，不同文法 → 不同树**

```lox
- 1 - 2     // 一元负号 vs 二元减，由 unary / term 分层消歧
```

**3. 空文件**

```text
Token: EOF only  →  AST: 空程序（零语句）
```

---

**一句话**：Parser 做「**搭骨架**」— 把 Token 串按文法拼成树；**不管**类型对不对、变量有没有定义（那是语义分析 / 运行时）。

**本书对应**：Part II **ch5**（AST 类型 + Visitor）· **ch6**（递归下降 Parser）· Part III clox **ch17** 起（编译表达式为字节码，内部另有编译器前端）。

**本仓库**：深入 [ch5 §5.2 实现语法树](../../part02_jlox/chapter05_representing-code/02-implementing-syntax-trees.md) · [ch6 §6.2 递归下降](../../part02_jlox/chapter06_parsing-expressions/02-recursive-descent-parsing.md) · **03** BYOC 语法分析章节

→ 下一节：[§2.1.3 中间表示（IR）](#231-中间表示intermediate-representations-ir)

---

### §2.1.3 中间表示（Intermediate Representations, IR）

| 项目 | 说明 |
|------|------|
| **位置** | 流水线**第三步**（接在 Parsing / 语义分析之后） |
| **输入** | **AST**（或已标注类型的 AST / HIR） |
| **输出** | **IR** — 更扁平、更接近执行的中间代码 |
| **核心动作** | **Lowering** — 把语法树「降维」成便于优化与后端的表示 |
| **为何需要** | 脱离源码括号/关键字形状；多 Pass 复用；多后端共享优化 |

**AST vs IR**

| | **AST** | **IR** |
|---|---------|--------|
| 形状 | **树**（嵌套表达式节点） | 常更**线性**（指令序列、基本块、SSA） |
| 贴近 | **源语言语法** | **执行模型**（栈、寄存器、内存） |
| 优化 | 可做，但遍历树较麻烦 | **Pass 流水线**的标准输入（LLVM、clox） |
| 本书例子 | jlox 直接解释 AST | clox **Chunk 字节码** · Rust **LLVM IR** |

```text
AST（语法树）  →  Lowering / 语义分析  →  IR  →  优化 Pass  →  代码生成
```

> **工业编译器常插入一步**：AST 上或 AST→IR 途中做**类型检查、作用域解析、常量折叠**（Rust：`rustc` 还有 **MIR** 再做 borrow check）。CI 原书 §2.1 从 Parsing 直接讲 IR，此处单独点出。

---

#### 例子 1 · 同一表达式：AST → clox 字节码 IR

**源码（接 §2.1.2）：**

```lox
1 + 2 * 3
```

**AST（树）：**

```text
Binary(+)
├── Literal(1)
└── Binary(*)
    ├── Literal(2)
    └── Literal(3)
```

**clox Chunk（线性 IR · 栈式 VM）** — 编译器 **ch17** 产出：

```text
; 常量池: [0]=1  [1]=2  [2]=3
0000  OP_CONSTANT    0    ; push 1
0002  OP_CONSTANT    1    ; push 2
0004  OP_CONSTANT    2    ; push 3
0006  OP_MULTIPLY           ; pop 3,2 → push 6
0007  OP_ADD                ; pop 6,1 → push 7
0008  OP_RETURN
```

**对比**：树变**指令流**；执行顺序由栈操作隐含，不再遍历 `Binary` 节点。

→ [ch14 何为 Bytecode](../../part03_clox/chapter14_chunks-of-bytecode/01-what-is-bytecode.md) · [ch17 发射字节码](../../part03_clox/chapter17_compiling-expressions/03-emitting-bytecode.md)

---

#### 例子 2 · jlox：AST 即「够用的 IR」

jlox **不单独建**字节码 / LLVM IR：

```text
Parser → AST  →  Visitor 解释执行（ch7）
```

| 优点 | 缺点 |
|------|------|
| 实现快、调试直观 | 节点分散、缓存不友好、间接跳转多 |
| 适合教学 Part II | clox Part III 才换紧凑 IR + VM |

**结论**：AST **可以视为一种前端 IR**；工业级编译器通常还会再降一层。

---

#### 例子 3 · Rust：多层 IR（概念）

**源码：**

```rust
fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

**简化流水线：**

```text
Rust 源码
  → AST（语法）
  → HIR（命名解析、类型推断后）
  → MIR（借用检查、关键优化）
  → LLVM IR（SSA，跨 crate 优化）
  → x86-64 机器码
```

**LLVM IR 片段（概念 · SSA）：**

```llvm
define i32 @add(i32 %a, i32 %b) {
entry:
  %sum = add i32 %a, %b    ; 每个 % 名只赋值一次 → SSA
  ret i32 %sum
}
```

**SSA（Static Single Assignment）**：每个「变量」只被赋值一次，便于**常量传播、死代码消除**等 Pass。

→ [04 Learn LLVM 17](../../../04_Learn-LLVM-17/README.md) · [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md)

---

#### 例子 4 · 常量折叠（优化发生在 IR 上）

**源码：**

```lox
print 2 + 3;
```

| 阶段 | 表示 |
|------|------|
| AST | `Print(Literal?)` 或 `Print(Binary(+, 2, 3))` |
| **优化后 IR** | 直接 `OP_CONSTANT 5`（编译期算完） |
| 未优化 | 仍 emit `OP_CONSTANT 2` · `OP_CONSTANT 3` · `OP_ADD` |

**要点**：优化 Pass 在 **IR** 上改写成更简单指令，不必回头改 AST 形状。

---

#### 例子 5 · 前端 IR vs 后端 IR

| 种类 | 贴近谁 | 本书 / Rust 例子 |
|------|--------|------------------|
| **前端 IR** | 源语言 | AST · HIR · clox 编译前的语法树 |
| **后端 IR** | 目标机器 / VM | clox **字节码 Chunk** · **LLVM IR** · BYOC **`cbc.ir`** |

```text
         前端 IR              后端 IR
jlox:    AST          →      （无，直接解释）
clox:    编译器内部树  →      Chunk 字节码
Rust:    MIR          →      LLVM IR
cbc:     ast 包       →      ir 包（见 03 BYOC ch2）
```

→ [03 BYOC · cbc 包结构](../../../03_Build-Your-Own-Compiler/chapter02_cflat-cbc/02-cbc-packages.md)

---

#### 例子 6 · IR 为「多后端」服务

```text
        ┌─→ x86-64 代码生成
LLVM IR ┼─→ ARM64 代码生成
        └─→ WebAssembly
```

同一套 **LLVM 优化 Pass** 作用于 IR，再各自 codegen — 前端（Rust）不必为每个 CPU 写优化。

clox 类比：字节码 IR 同一套，VM 在 Windows/macOS/Linux 上解释同一 Chunk（**假想栈机** = 可移植目标）。

---

#### 例子 7 · 易错边界（自测用）

**1. IR ≠ 机器码**

```text
OP_ADD 是 clox 字节码；CPU 读的是 0x48 0x01… 这类 x86 指令
```

**2. IR 仍在编译期存在**

```text
LLVM IR 在 cargo build 时生成并优化；链接后 IR 文件不在运行时读取
```
→ 对照 [05 编译期 LLVM vs Runtime](./05-compile-time-llvm-vs-runtime.md)

**3. 有 IR 不一定有 VM**

```text
Rust → LLVM IR → 机器码（无 VM）
clox → 字节码 IR → VM 解释
```

**4. AST 可以跳过显式 IR（捷径）**

```text
jlox 树遍历 = §2.2 捷径之一
```

---

**一句话**：IR 做「**换坐标系**」— 从语法树换成指令/SSA 流，方便优化 Pass 和后端 codegen；**仍属编译期**，不是 Runtime。

**本书对应**：jlox 以 **AST** 为终点 IR · clox **ch14～17**（Chunk 字节码）· **04 Learn LLVM 17**（LLVM IR / Pass）

**本仓库**：RFR [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md) · [03-2 OS/LLVM 内存布局](../../../../02-RFR/Chapter-01-Foundations/03-2-os-memory-layout.md)（`alloca`/heap 常出现在 LLVM IR 层）

→ 下一节：[代码生成（Code Generation）](#代码生成code-generation)

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
