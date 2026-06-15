# 第 7 章 · Evaluating Expressions（求值表达式）

> 在线：[evaluating-expressions.html](https://craftinginterpreters.com/evaluating-expressions.html) · 中文：[第 7 章 表达式求值](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch06 Parsing](../chapter06_parsing-expressions/06-parsing-expressions.md)

---

## 本章定位

解释器**「睁开眼睛」**：从静态 AST → **真正执行并算出结果**。

| ch6 | ch7 | ch8 |
|-----|-----|-----|
| Token → AST | **AST → 值** | 语句 + 变量 + 状态 |

| 小节 | 主题 |
|------|------|
| **§7.1** | Lox 值 ↔ Java `Object` 映射 |
| **§7.2** | `Interpreter` 实现 `Expr.Visitor` · 后序遍历 |
| **§7.3** | 运行时类型错误 · `RuntimeError` |
| **§7.4** | `interpret()` · `stringify()` · REPL 不退出 |

---

## §7.1 表示值（Representing Values）

**问题**：Lox **动态类型** vs Java **静态类型**。

**方案**：用 Java **`Object`** 存任意 Lox 运行时值：

| Lox | Java 表示 |
|-----|-----------|
| `nil` | `null` |
| Number | `Double` |
| Boolean | `Boolean` |
| String | `String` |

后续 ch8 变量、ch10 函数等仍在此模型上扩展。

---

## §7.2 求值表达式（Evaluating Expressions）

复用 ch5 **Visitor**：`Interpreter implements Expr.Visitor<Object>`。

### 遍历方式：后序（post-order）

- 先求**子表达式**，再应用**当前节点**运算符。
- 与 AST 结构一致：Binary 先 `left`、`right`，再算 `op`。

### 各节点要点

| 节点 | 行为 |
|------|------|
| **Literal** | 原样返回字面量值 |
| **Grouping** | 递归求内部表达式 |
| **Unary** | `-` 数值取负；`!` 用 **truthy** 规则 |
| **Binary** | 算术 / 比较；**左操作数先求值再右**（求值顺序） |

### Truthy（真假值）

- 动态语言常见：并非只有 `true/false` 参与逻辑。
- `!` 等对操作数做 **truthy** 判定（ch9 `and/or` 会延伸）。

**对照 ch3**：§3.4 表达式 · §3.2 动态类型在运行时才落地。

---

## §7.3 运行时错误（Runtime Errors）

- 非法运算（如 `-"muffin"`）→ 不应让 Java **`ClassCastException`** 崩掉整个 VM。
- **类型检查**后再运算；失败 → 抛自定义 **`RuntimeError`**（带**行号**）。

| 阶段 | 错误类型 |
|------|----------|
| 词法 | Scanner / ch4 |
| 语法 | `ParseError` / ch6 |
| **运行时** | **`RuntimeError`** / 本章 |

---

## §7.4 接入解释器（Hooking Up the Interpreter）

| API | 作用 |
|-----|------|
| **`interpret(Expr)`** | 对外入口：解释一棵表达式 AST |
| **`stringify(Object)`** | Java 值 → Lox 风格**字符串**（供 `print` / REPL 显示） |

- 主程序 / REPL **捕获 `RuntimeError`** → 报错后继续提示符，**不退出**进程。

```text
parse() → Expr
interpret(expr) → Object
stringify(result) → 用户可见输出
```

---

## 本章速记

```text
§7.1  Object 存 Lox 值 · nil→null · Double/Boolean/String
§7.2  Visitor 后序求值 · truthy · 二元左右顺序
§7.3  RuntimeError + 行号 · 不崩 JVM
§7.4  interpret · stringify · REPL 捕获
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **8** | [chapter08 · Statements and State](../chapter08_statements-and-state/) | **Stmt** · `var` · **Environment** |

---

## 阅读进度

- [x] §7.1～§7.4 结构梳理（本章笔记）
- [ ] 跟书实现 `Interpreter` + REPL 算式
- [ ] 本章 Challenges
