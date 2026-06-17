# 第 2 章 · A Map of the Territory（领域地图） · §2.1 语言的组成部分（The Parts of a Language）

← [本章目录](./README.md) · 上一节：[00-overview.md](./00-overview.md) · 下一节：[02-shortcuts-and-alternate-routes.md](./02-shortcuts-and-alternate-routes.md)

---

完整 **Pipeline**（那座「山」上的途径点）：

```text
Source Code（源码）
    ↑ 上山 · 前端
    Scanning / Lexing     → Tokens
    Parsing               → AST
    Static Analysis       → 绑定 / 类型检查 / 符号表
    Intermediate Rep (IR) → 便于优化与后端
    Optimization          → 等价改写 IR
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

> **原书编号 §2.1.4**；原书 §2.1.3 静态分析见 [§2.1.4](#241-静态分析static-analysis)。

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

→ 下一节：[§2.1.4 静态分析](#241-静态分析static-analysis)

---

### §2.1.4 静态分析（Static Analysis）

> **原书编号 §2.1.3** · 流水线位置：Parsing **之后**、Lowering 到 IR **之前**（概念上在 §2.1.3 IR 降维之前）。

| 项目 | 说明 |
|------|------|
| **位置** | 前端末段 / **middle end** 入口 |
| **输入** | **AST**（只有语法形状） |
| **输出** | 带语义信息的 AST、**符号表**，或更低层 IR |
| **核心动作** | **Binding / 类型检查 / 作用域解析** |
| **不做** | 生成机器码、运行程序 |

**Parsing 之后仍不知道什么**

```lox
print a + b;
```

Parser 只得到 `Print(Binary(+, Var(a), Var(b)))` — **`a`、`b` 是谁？在哪定义？什么类型？** 静态分析回答这些。

---

#### 例子 1 · 名字绑定（Binding / Resolution）

**源码：**

```lox
var x = 1;
{
  print x;
}
```

**分析结果（概念）：**

```text
符号表:
  x → 全局槽 #0  （或当前环境 depth=0, slot=0）

AST 节点 Var("x") 附加属性:
  resolved: Global #0
  或 distance: 1, slot: 0   （jlox ch11 Resolver）
```

**jlox**：`Resolver` 遍历 AST，给每个 `Var` 写入 **upvalue distance** → 运行时不再按名字查找。

→ [jlox ch11 Resolving and Binding](../../part02_jlox/chapter11_resolving-and-binding/README.md)

---

#### 例子 2 · 作用域错误（静态报错）

**源码：**

```lox
{
  print inner;
  var inner = 1;
}
```

| 阶段 | 结果 |
|------|------|
| Scanner / Parser | ✅ 语法合法 |
| **静态分析** | ❌ `inner` 在声明前使用 — **语义错误** |

**Rust 对照：**

```rust
let x = y;  // ❌ 编译期：cannot find value `y`
let y = 1;
```

---

#### 例子 3 · 类型检查（静态类型语言）

**Lox** — **动态类型**，静态分析阶段**不做**完整类型检查；`"hi" + 3` AST 合法，**运行时才报错**。

**Rust** — **静态类型**，同一阶段拒绝：

```rust
let s: &str = "hi";
let n: u32 = s + 3;   // ❌ 编译期 type mismatch
```

| | **Lox / Python 风格** | **Rust / C 风格** |
|---|----------------------|-------------------|
| 类型 | 运行时查 | **编译期**查 |
| 静态分析重点 | 绑定、闭包捕获 | 绑定 + **类型** + 借用（MIR 上 borrowck） |

---

#### 例子 4 · 信息存哪

| 方式 | 例子 |
|------|------|
| **AST 节点附加字段** | `Var.resolved_slot` · `Expr.ty` |
| **符号表（side table）** | `HashMap<Ident, Decl>` |
| **换成新 IR** | AST → HIR/MIR（Rust）— 语义更直白 |

```text
Parser:     AST（空语义字段）
Resolver:   AST + distance/slot
Typeck:     AST/HIR + 每个 expr 的 Ty
Lowering:   → §2.1.3 IR
```

---

#### 例子 5 · 前端 / middle end / 后端（原书分界）

```text
Scan · Parse · Static Analysis     ← 前端（+ 部分 middle end）
IR · Optimization                  ← middle end
Code Gen · VM                      ← 后端
Runtime                            ← 运行期
```

**「编译之山」山顶**：静态分析完成时，编译器对程序**语义**有完整鸟瞰，再开始「下山」降 IR、优化、出码。

---

#### 例子 6 · 易错边界（自测用）

**1. 静态分析 ≠ 优化** — 前者**理解**程序；后者在保持语义下**改写**程序。

**2. jlox 树遍历仍要做 Resolver** — 静态分析不必等 IR；可在 AST 上完成。

**3. `a + b` 类型兼容** — Lox 运行期；Rust `i32 + &str` 编译期挂。

---

**一句话**：静态分析给 AST **填语义** — 谁是谁、能否相加、变量在哪；**仍属编译期**，不产生可执行文件。

**本书对应**：jlox **ch11 Resolver** · clox 在编译期做 **Upvalue 解析**（ch22 等）

**本仓库**：Rust borrowck → [Nomicon 03 Lifetime](../../../../03-Rust_Nomicon/03_Lifetime_Variance/README.md) · BYOC **`type` 包**

→ 下一节：[§2.1.5 优化](#251-优化optimization)

---

### §2.1.5 优化（Optimization）

> **原书编号 §2.1.5** · 流水线位置：已有 IR（§2.1.3）且理解语义（§2.1.4）**之后**，代码生成 **之前**。

| 项目 | 说明 |
|------|------|
| **输入** | **IR**（或带类型的 AST/HIR） |
| **输出** | 语义等价、**更高效**的 IR |
| **核心动作** | 在**不改变程序含义**前提下改写 |
| **典型 Pass** | 常量折叠、死代码消除、内联、循环展开 |

```text
IR（未优化）  →  Pass₁  →  Pass₂  →  …  →  IR（优化后）  →  §2.1.6 代码生成
```

---

#### 例子 1 · 常量折叠（原书经典）

**源码：**

```c
pennyArea = 3.14159 * (0.75 / 2) * (0.75 / 2);
```

**优化后（编译期算完）：**

```c
pennyArea = 0.4417860938;
```

**clox IR 对照** — `print 2 + 3;`：

```text
未优化:  OP_CONSTANT 2 · OP_CONSTANT 3 · OP_ADD
优化后:  OP_CONSTANT 5
```

→ 已在 [§2.1.3 例子 4](#例子-4--常量折叠优化发生在-ir-上)

---

#### 例子 2 · 死代码消除（DCE）

**源码：**

```rust
fn foo() -> i32 {
    let dead = expensive();
    42
}
```

**优化后 IR**：若 `dead` 无副作用，`expensive()` 调用可被 **DCE** 删掉 — 用户仍得到 `42`。

---

#### 例子 3 · 内联（Inlining）

**源码：**

```rust
#[inline]
fn sq(x: i32) -> i32 { x * x }
let y = sq(5);
```

**优化后（概念）**：`let y = 5 * 5;` — 省掉 call 开销，利于后续常量折叠。  
**HFT 热路径**：静态分发 + 内联 → 接近手写 C（RFR ch2 · `impl Trait` / 单态化）。

→ [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md)

---

#### 例子 4 · LLVM O0 vs O3

同一 Rust 函数，`llvm-opt` 级别不同 → IR/机器码差异巨大：

| 级别 | 典型行为 |
|------|----------|
| **O0** | 少优化，便于调试，体积大 |
| **O3** | 积极内联、向量化、DCE |

→ [04 Learn LLVM 17 · ch07 IR 优化](../../../04_Learn-LLVM-17/part02_src_to_machine/chapter07_ir_optimize/README.md) · `ir_samples/optimize_compare/`

---

#### 例子 5 · 本书对优化的态度

| 路线 | 优化在哪做 |
|------|------------|
| **Lua / CPython** | 编译期 IR **较朴素**；性能靠 **runtime**（JIT、专用结构） |
| **clox 全书** | 编译期字节码 **几乎不优化**；**ch30** 才做 **VM 实现层**微优化（NaN boxing、哈希探测） |
| **Rust + LLVM** | 大量优化在 **LLVM Pass**（middle end） |

**区分**：

```text
编译期优化  = 改 IR / 机器码（本节）
运行期优化  = clox ch30 · JVM JIT · PGO
```

---

#### 例子 6 · 易错边界（自测用）

**1. 优化必须保持语义** — 不能 `2+3` _fold 成 `6` 若会改变溢出行为（语言规则约束）。

**2. 没 IR 也能做少量优化** — AST 上常量折叠；但工业级 Pass 多针对 **SSA IR**。

**3. `-C opt-level=0` 不是「没编译」** — 仍 codegen，只是 Pass 少。

---

**一句话**：优化 = **等价改写 IR**，让后面的代码生成产出更快/更小的机器码；**仍属编译期**。

**本书对应**：概念本节 · clox **ch30 Optimization**（VM 侧）· LLVM **04 ch07**

→ 下一节：[§2.1.6 代码生成](#261-代码生成code-generation)

---

### §2.1.6 代码生成（Code Generation）

> **原书编号 §2.1.6** · 流水线 **后端下山**：IR → 可执行形式。

| 项目 | 说明 |
|------|------|
| **输入** | 优化后的 **IR** |
| **输出** | **机器码** 或 **字节码** |
| **核心动作** | 把中间表示**映射**到目标指令集 |
| **关键决策** | **真 CPU** vs **假想 VM**（字节码） |

```text
IR  →  Code Generator  →  机器码（ELF）  或  字节码（Chunk）
```

---

#### 例子 1 · 两条后端路线

| 路线 | 产出 | 谁执行 | 本书 / Rust |
|------|------|--------|-------------|
| **AOT 原生** | x86-64 / ARM 机器码 | **OS 加载 → CPU 直跑** | **Rust** · `rustc` + LLVM |
| **字节码 + VM** | `OP_*` 指令流 | **VM 解释**（或 JIT） | **clox** · JVM · CPython |

```text
                    ┌─→ 机器码 ─→ CPU
优化后 IR ── CodeGen ┤
                    └─→ 字节码 ─→ VM（§2.1.7）
```

---

#### 例子 2 · clox：表达式 → 字节码（接 §2.1.3）

**源码：** `1 + 2 * 3`

**CodeGen（ch17 `emit`）产出：**

```text
OP_CONSTANT 0    ; 1
OP_CONSTANT 1    ; 2
OP_CONSTANT 2    ; 3
OP_MULTIPLY
OP_ADD
OP_RETURN
```

编译器 **递归遍历语法结构**，对每个子表达式 emit，再 emit 组合 opcode — 不是 CPU 汇编，但是 **clox 的后端 codegen**。

→ [ch17 Emitting Bytecode](../../part03_clox/chapter17_compiling-expressions/03-emitting-bytecode.md)

---

#### 例子 3 · Rust / LLVM：IR → 机器码（概念）

**LLVM IR：**

```llvm
define i32 @add(i32 %a, i32 %b) {
entry:
  %sum = add i32 %a, %b
  ret i32 %sum
}
```

**Codegen 后（x86-64 概念 · 极度简化）：**

```asm
add:
    lea  eax, [rdi + rsi]   ; 或 add edi, esi 等，视 calling convention
    ret
```

**链接**：多个 `.o` + std/Tokio **已编译的机器码** → 单一 **ELF** 可执行文件。

→ [05 编译期 LLVM vs Runtime](./05-compile-time-llvm-vs-runtime.md)

---

#### 例子 4 · 原生 code gen 的代价

| 优点 | 缺点 |
|------|------|
| **最快**（无 VM 解释层） | 指令集复杂（x86 历史包袱） |
| OS 直接 `exec` | **绑定架构** — x86 二进制不能直接在 ARM 跑 |
| HFT 默认路线 | 编译器后端开发量大 → 故用 **LLVM** |

**1960s 对策**：不生成真机器码 → 生成 **p-code / bytecode**（§2.1.7 VM）。

---

#### 例子 5 · BYOC cbc 对照

```text
cbc IR  →  codegen 包  →  x86 汇编文本  →  汇编器  →  机器码
```

→ [03 BYOC · 四阶段编译](../../../03_Build-Your-Own-Compiler/chapter01_start/02-four-compiler-stages.md)

---

#### 例子 6 · 易错边界（自测用）

**1. CodeGen ≠ Runtime** — 生成代码是**编译期**；GC/async 调度是**运行期**。

**2. 字节码也是 code gen 产物** — 只是目标 ISA 是 **假想栈机**，不是 x86。

**3. `cargo build` 已含 codegen** — 不是只到 IR 为止。

**4. JIT** — 运行期再做 code gen（§2.2.4 捷径）；AOT 在 build 时完成。

---

**一句话**：代码生成 = **把 IR 翻译成能跑的形式** — 要么 CPU 机器码，要么给 VM 的字节码。

**本书对应**：clox **ch17～24**（编译各类语法到 Chunk）· **04 LLVM** 后端 · **03 BYOC** x86 发射

→ 下一节：[§2.1.7 虚拟机](#217-虚拟机virtual-machine)

---

### §2.1.7 虚拟机（Virtual Machine）

> **原书编号 §2.1.7** · 仅 **字节码路线**需要；原生机器码路线跳过本节。

| 项目 | 说明 |
|------|------|
| **输入** | **字节码 Chunk**（§2.1.6 产出） |
| **输出** | 程序执行结果 |
| **核心动作** | **模拟**假想指令集 — fetch-decode-execute 循环 |
| **替代方案** | 字节码 **AOT 转 native**（每架构一个小后端，仍复用前端） |

---

#### 例子 1 · VM 主循环（clox）

```c
for (;;) {
  uint8_t op = *vm.ip++;
  switch (op) {
    case OP_ADD:      /* pop 2, push sum */ break;
    case OP_RETURN:   return;
  }
}
```

```text
ip 指向 Chunk.code  →  读 opcode  →  改栈  →  ip++
```

→ [ch15 指令执行机](../../part03_clox/chapter15_a-virtual-machine/01-an-instruction-execution-machine.md)

---

#### 例子 2 · 执行 `1 + 2 * 3` 的栈变化

```text
OP_CONSTANT 1   stack: [1]
OP_CONSTANT 2   stack: [1, 2]
OP_CONSTANT 3   stack: [1, 2, 3]
OP_MULTIPLY     stack: [1, 6]
OP_ADD          stack: [7]
OP_RETURN
```

---

#### 例子 3 · VM vs 原生

| | **VM 解释** | **原生机器码** |
|---|-------------|----------------|
| 速度 | 较慢（每条 opcode 用 C 模拟） | 快 |
| 移植 | **同一份字节码**跨平台 | 每架构一份二进制 |
| 本书 | **clox** | Rust 默认 |

**JIT 折中**：HotSpot / V8 运行期把热点字节码 **再 codegen** 成机器码（§2.2.4）。

---

#### 例子 4 · 易错边界

**1. VM 是 Runtime 组件，不是 LLVM** — 字节码在 build 时生成；VM 在 `./app` 时运行。

**2. JVM / Python / clox 都是 VM 路线** — Rust 默认不是。

---

**一句话**：VM = **软件实现的 CPU**，专门执行你的字节码。

**本书对应**：Part III **clox ch14～15** 起 · ch30 VM 微优化

→ 下一节：[§2.1.8 运行时](#218-运行时runtime)

---

### §2.1.8 运行时（Runtime）

> **原书编号 §2.1.8** · 程序**被加载执行后**才活跃的服务层。

| 项目 | 说明 |
|------|------|
| **何时** | OS 加载二进制 **之后** |
| **职责** | GC、动态类型检查、`instanceof`、模块加载、**async 调度**… |
| **代码从哪来** | 同样经 **§2.1.6 codegen** 编进二进制，或住在 **VM 进程**里 |

---

#### 例子 1 · 两类部署

| | **编译进可执行文件** | **住在 VM 里** |
|---|---------------------|----------------|
| **例子** | Go runtime · Rust std · clox 内嵌 GC 代码 | Java HotSpot · CPython 解释器 |
| **启动** | `_start` → runtime init → `main` | `java` / `python` 先启 VM，再加载字节码 |

---

#### 例子 2 · clox Runtime 服务

| 服务 | 章节 |
|------|------|
| **值表示 / 动态类型** | ch18 |
| **堆分配** | ch19 |
| **GC Mark-Sweep** | ch26 |
| **方法 / 闭包 / 类** | ch24～28 |

```lox
class A {}
var o = A();
print o;           // 运行期：类型标签、分配、toString
```

→ [ch26 GC](../../part03_clox/chapter26_garbage-collection/README.md)

---

#### 例子 3 · jlox vs Rust

| | **jlox** | **Rust** |
|---|----------|----------|
| GC | 借 **Java GC** | 无 GC（所有权） |
| 动态类型 | 运行期查 `+` 两侧类型 | 编译期已定 |
| 「Runtime」 | JVM + jlox 解释器 | 极薄 **std** + 可选 **Tokio**（库级） |

---

#### 例子 4 · 易错边界

**1. Runtime 源码也经 LLVM 编译** — 见 [05 编译期 LLVM vs Runtime](./05-compile-time-llvm-vs-runtime.md)

**2. Runtime ≠ VM** — VM 执行字节码；Runtime 是更广的「运行期服务」集合（GC、调度等），可重叠。

**3. `no_std`** — 刻意剥离大部分 Rust runtime 服务。

---

**一句话**：Runtime = 程序跑起来后的**管家**（内存、类型、调度）；与 **LLVM 翻译官**（编译期）彻底分开。

**本书对应**：jlox 借 Java runtime · clox **ch18～26** · Rust [RFR 第 1 章内存](../../../../02-RFR/Chapter-01-Foundations/README.md)

→ **Rust/HFT 分层对照**：[04-rust-hft-编译流水线对照.md](./04-rust-hft-编译流水线对照.md) · [05 编译期 LLVM vs Runtime](./05-compile-time-llvm-vs-runtime.md)

---
