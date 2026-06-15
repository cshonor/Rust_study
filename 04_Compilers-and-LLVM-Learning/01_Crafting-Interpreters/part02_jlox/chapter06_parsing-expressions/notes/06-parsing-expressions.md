# 第 6 章 · Parsing Expressions（解析表达式）

> 在线：[parsing-expressions.html](https://craftinginterpreters.com/parsing-expressions.html) · 中文：[第 6 章 解析表达式](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch05 Representing Code](../chapter05_representing-code/notes/05-representing-code.md)

---

## 本章定位

**Part II 第一个重要里程碑**：实现 **Parser**，把 **Token 流** → **AST**（反映语法嵌套）。

| 上一章 | 本章 | 下一章 |
|--------|------|--------|
| ch5 定义 AST + Visitor | **ch6 构造 Expr 树** | ch7 **运行**树（求值） |

| 小节 | 主题 |
|------|------|
| **§6.1** | 歧义 · 优先级 · 结合性 · 分层语法 |
| **§6.2** | 递归下降 · 规则→方法 · `while` 建 Binary |
| **§6.3** | 恐慌模式 · 同步恢复 |
| **§6.4** | `parse()` 接入解释器 |

---

## §6.1 歧义与解析游戏（Ambiguity and the Parsing Game）

### 语法歧义

- CFG 若不够严谨 → **同一串源码**可对应**多种**合法 parse。
- 例：`6 / 3 - 1`
  - `(6 / 3) - 1` → **1**
  - `6 / (3 - 1)` → **3**
- 不同语法树 → **不同语义** → 必须消除歧义。

### 优先级（Precedence）

- 决定**哪个运算符先结合**（先算 `*` 还是先算 `+`）。
- 在 AST 中：**优先级越高 → 节点在树上越深**（更靠近叶子）。

### 结合性（Associativity）

- 同级运算符：**左结合**（`a - b - c` = `(a - b) - c`）或 **右结合**（幂运算常见右结合）。

### 分层语法（Lox 表达式，从低到高）

| 层级 | 规则名 | 典型运算符 |
|------|--------|------------|
| 低 | **equality** | `==` `!=` |
| ↑ | **comparison** | `>` `>=` `<` `<=` |
| ↑ | **term** | `+` `-` |
| ↑ | **factor** | `*` `/` |
| ↑ | **unary** | `!` `-`（一元） |
| 高 | **primary** | 字面量、`()`、标识符… |

**原则**：低优先级规则在语法上**先被调用**，内部再调用更高优先级 → 树结构自然正确。

**对照 ch5 BNF**：§5.1 的蓝图 · 本章落地为 **Parser 方法链**。

---

## §6.2 递归下降解析（Recursive Descent Parsing）

### 为何选递归下降

| 特点 | 说明 |
|------|------|
| **自顶向下** | 从起始符号 `expression` 往下拆 |
| **手写友好** | 每条文法规则 ≈ 一个 Java 方法 |
| **速度快、健壮** | 工业界常用（许多生产编译器前端） |
| **错误处理** | 可精细控制（§6.3） |

### 规则 → 代码

```text
equality   →  equality()   方法
comparison →  comparison() 方法
term       →  term()       方法
…
```

### Token 流辅助方法

| 方法 | 作用 |
|------|------|
| **`match(type, …)`** | 若当前 Token 类型匹配 → **消耗**并返回 true |
| **`check(type)`** | **只看**当前类型，不消耗 |
| **`advance()`** | 读下一个 Token |
| **`previous()`** | 刚消耗的那个 Token（常取 literal） |

### 构建 `Expr.Binary` 与左结合

- 二元层（如 `term` 处理 `+` `-`）典型模式：

```text
Expr expr = factor();           // 左操作数
while (match(PLUS, MINUS)) {
  Token op = previous();
  Expr right = factor();
  expr = new Binary(expr, op, right);  // 左结合：新节点以旧 expr 为左子
}
return expr;
```

- **`while` + 左子不断「长高」** → 自然实现**左结合**（`1 - 2 - 3`）。

**验证**：用 ch5 **`AstPrinter`** 打印 `(1 - 2 - 3)` 的树形。

---

## §6.3 语法错误（Syntax Errors）

解析器遇到非法源码时的**两个目标**（略矛盾，需平衡）：

| 目标 | 含义 |
|------|------|
| **多报独立错误** | 一次编译尽量列出多处真实错误 |
| **少级联错误** | 不因前一个错误状态错乱而刷一堆「幽灵错误」 |

### 恐慌模式（Panic mode）

- 遇到**意料之外 Token** → 抛 **`ParseError`**。
- **停止**当前语法树分支的构建，进入恐慌。

### 同步（Synchronization）

- **捕获** `ParseError` → **丢弃 Token** 直到**同步点**：
  - 分号 **`;`**
  - 或语句起始关键字：`class` `fun` `var` `for` `if` `while` `print` …
- 状态重置 → **继续**解析下一条语句，寻找后续真实错误。

```text
错误 → panic → synchronize() → 丢弃直到边界 → 继续 parse
```

**本仓库**：`rustc` 错误恢复更复杂；Lox 教学版 panic mode 是经典入门模板。

---

## §6.4 组合起来（Wiring up the Parser）

- 编写 **`parse()`** 启动解析。
- **早期阶段**：只解析并返回**单个表达式**（语句、声明留 ch8+）。
- 捕获 **`ParseError`** → 返回 **`null`**，避免解释器在坏语法上崩溃或挂起。

```text
Scanner.scan() → List<Token>
Parser.parse() → Expr 或 null
（ch7）Interpreter.interpret(expr) → 值
```

---

## 流水线里程碑

```text
ch4  Scanning      Token[]
ch5  AST 类型
ch6  Parsing       Expr 树     ← 本章：一维 → 结构化
ch7  Evaluating    运行树
ch8+ 语句、作用域、控制流…
```

**clox**：不共用 jlox 的 `Parser` 类；**ch17** 用 **Pratt 解析器** + 编译到字节码（另一种手写 parser 风格）。

---

## 本章速记

```text
§6.1  歧义 · precedence/associativity · 分层 equality→…→primary
§6.2  递归下降 · 一规则一方法 · while 建 Binary 左结合
§6.3  panic mode · synchronize 到 ; 或关键字
§6.4  parse() → Expr | null
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **7** | [chapter07 · Evaluating Expressions](../chapter07_evaluating-expressions/) | **Visitor 解释器** · AST → 值 |
| **8** | Statements and State | 语句 · 环境 · 副作用 |

ch7 前建议：用 **AstPrinter** 对 REPL 输入验几棵表达式树。

---

## 自测 / 对照（可选）

- [ ] 画出 `1 + 2 * 3` 的 AST（哪个运算符更深？）。
- [ ] 说明 `6 / 3 - 1` 在**左结合**下对应哪棵树。
- [ ] 写出 `term()` 里 `while (match(PLUS, MINUS))` 循环在做什么。
- [ ] 若源码 `var x = ;`，同步点会在哪里恢复？（概念上）

---

## 阅读进度

- [x] §6.1～§6.4 结构梳理（本章笔记）
- [ ] 跟书实现 `Parser` + `AstPrinter` 联调
- [ ] 本章 Challenges
