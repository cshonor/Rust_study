# 第 5 章 · Representing Code（表示代码 / AST）

> 在线：[representing-code.html](https://craftinginterpreters.com/representing-code.html) · 中文：[第 5 章 表示代码](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch04 Scanning](../chapter04_scanning/04-scanning.md)

---

## 本章定位

**ch4** 产出线性 **Token 流**；**ch5** 定义如何把 Token 组织成反映**语法嵌套**的树 → **抽象语法树（AST）**。

| 输入（概念上） | 输出（本章） |
|----------------|--------------|
| Token 序列 | **AST 节点类型** + **Visitor** 基础设施 |

**ch6 Parsing** 将把 Token **真正构造**成这棵树。

| 小节 | 主题 |
|------|------|
| **§5.1** | 上下文无关文法（CFG）· BNF · Lox 表达式语法 |
| **§5.2** | Java `Expr` 层次 · `GenerateAst.java` |
| **§5.3** | Visitor · 双重分派 |
| **§5.4** | `AstPrinter` 实战 |

**附录 II**：[`appendix02_generated-ast-classes`](../../backmatter/appendix02_generated-ast-classes/) · 生成代码参考。

---

## §5.1 上下文无关文法（Context-Free Grammars）

### 为何需要 CFG

- **正则表达式 / 扫描器**只能处理**线性**、**不嵌套**的结构。
- 表达式如 `1 + (2 * 3)` 需要**任意嵌套** → 超出正则能力 → 交给 **Parser** + **CFG**。

### 文法规则与 BNF

- **上下文无关文法（CFG）**：用产生式描述「非终结符 → 符号串」。
- **巴科斯范式（BNF）**：形式化书写语法规则的常用记法。

### 增强符号（语法糖）

| 符号 | 含义 |
|------|------|
| `\|` | 分支（或） |
| `*` | 重复 0 次或多次 |
| `+` | 重复 1 次或多次 |
| `?` | 可选（0 或 1 次） |

用这些符号精确定义 **Lox 表达式**的语法蓝图（与 ch3 语义对应，ch6 按此实现 parser）。

**本仓库**：**03**《自制编译器》JavaCC · **附录 I** 完整 Lox 语法 · RFR 第 7 章宏（语法即数据的不同层面）。

---

## §5.2 实现语法树（Implementing Syntax Trees）

### Expr 类层次

- 基类抽象 **`Expr`**。
- 每种语法形式一个**子类**，例如：

| 节点类 | 对应语法 |
|--------|----------|
| `Binary` | 二元运算 `left op right` |
| `Unary` | 一元运算 `op right` |
| `Literal` | 字面量 |
| … | ch6 后还会扩展（分组、变量等） |

### 元编程：`GenerateAst.java`

- 手写大量字段 + 构造函数的 AST 类**繁琐且易错**。
- 作者用 **几十行 Java 脚本** `GenerateAst.java` **生成**全部 AST 源码 → 「让机器写无聊代码」。
- 运行脚本 → 输出 `Expr.java` 等文件（附录 II 含生成结果）。

```text
BNF 规则（人读）  →  GenerateAst（机器写）  →  Java AST 类（编译用）
```

**Rust 对照**：`syn` / 手写 enum · **proc-macro** 生成 AST —— 同一「元编程减样板」思路（RFR ch7）。

---

## §5.3 操作树（Working with Trees）

解释器生命周期里要对 AST 做多种**操作**：

| 操作示例 | 章节 |
|----------|------|
| **解释执行** | ch7 Evaluating |
| 类型检查 | （Lox 动态类型，运行时做） |
| 打印 / 调试 | §5.4 AstPrinter |

若把每种逻辑都写进节点类 → **臃肿、难扩展**。

### 访问者模式（Visitor Pattern）

- 把「对每种节点的操作」抽到**独立的 Visitor 类**。
- AST 节点只负责 **`accept(visitor)`** → 回调到 `visitor.visitXxx(this)`。

### 双重分派（Double Dispatch）

1. 第一次分派：`expr.accept(visitor)` → 根据**节点类型**选方法。
2. 第二次分派：`visitor.visitBinary(node)` → 根据 **Visitor 实现**选行为。

```text
新增一种操作  →  新建 Visitor 子类  →  不改 AST 节点类
新增一种节点  →  改 Expr + GenerateAst + 所有 Visitor  （较少发生）
```

**本仓库**：编译器里常见 **IR Visitor** · LLVM Pass 遍历 SSA —— 同族「树/图 + 访问」思想。

---

## §5.4 一个（不太）漂亮的打印机（AstPrinter）

- 实现 **Visitor 接口** 的 **`AstPrinter`**。
- 遍历 AST，输出 **Lisp 风格**带括号字符串：

```text
1 + 2 * 3   →   (+ 1 (* 2 3))
```

| 用途 | 说明 |
|------|------|
| 解释执行 | **非必需** |
| **调试 parser** | **极有用** —— 验证 ch6 是否建对了树 |

**建议**：写 ch6 时随时 `new AstPrinter().print(expr)` 对照预期。

---

## 流水线位置

```text
ch4  Token 流
ch5  AST 类型 + Visitor     ← 本章
ch6  Parser：Token → Expr 树
ch7  Interpreter：Visitor 求值
```

**clox**：不在 Java 里建相同 AST 类；编译器内部结构不同，但 **「中间树状表示 → 遍历/生成」** 思路相通（ch17 起）。

---

## 本章速记

```text
§5.1  正则不够 → CFG/BNF · | * + ?
§5.2  Expr 子类 · GenerateAst 生成代码
§5.3  Visitor + 双重分派 · 操作与节点分离
§5.4  AstPrinter → (+ 1 (* 2 3)) 验 parser
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **6** | [chapter06 · Parsing Expressions](../chapter06_parsing-expressions/) | **递归下降** · Token → AST |
| **7** | Evaluating | Visitor **解释执行** |

动手 ch6 前：运行 **`GenerateAst`** 生成 AST 类 · 扫一眼 **附录 II**。

---

## 自测 / 对照（可选）

- [ ] 用 BNF 写一条 Lox **二元表达式**规则（含 `*` 重复）。
- [ ] 解释：为何 `1 + (2 * 3)` 不能只用正则扫描器完成，还需要 parser？
- [ ] 画双重分派：`Binary.accept(interpreter)` 如何调到 `Interpreter.visitBinary`？
- [ ] 对 `print 1 + 2;` 的预期 AstPrinter 输出是什么？（需 ch6 后才有完整语句树）

---

## 阅读进度

- [x] §5.1～§5.4 结构梳理（本章笔记）
- [ ] 运行 `GenerateAst.java` / 对照附录 II
- [ ] 实现或阅读 `AstPrinter`
- [ ] 本章 Challenges
