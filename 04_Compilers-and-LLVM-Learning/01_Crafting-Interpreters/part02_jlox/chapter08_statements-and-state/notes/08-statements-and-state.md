# 第 8 章 · Statements and State（语句与状态）

> 在线：[statements-and-state.html](https://craftinginterpreters.com/statements-and-state.html) · 中文：[第 8 章 表达式和状态](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch07 Evaluating](../chapter07_evaluating-expressions/notes/07-evaluating-expressions.md)

---

## 本章定位

ch7 像**计算器**（单次表达式）；ch8 引入 **语句 + 变量** → 解释器有**记忆（状态）**。

| 概念 | ch7 | ch8 |
|------|-----|-----|
| 目的 | 表达式 **求值** | 语句 **副作用** |
| AST | `Expr` | **`Stmt`** + `Expr.Assign` 等 |
| 存储 | — | **`Environment`**（HashMap） |

| 小节 | 主题 |
|------|------|
| **§8.1** | `Stmt` · 表达式语句 · `print` |
| **§8.2** | `var` 声明 · 默认 `nil` |
| **§8.3** | `Environment` · define / lookup |
| **§8.4** | 赋值 `=` · l-value · `Expr.Assign` |
| **§8.5** | `{}` 块 · **环境链** · shadowing |

---

## §8.1 语句（Statements）

| 对比 | 表达式 | 语句 |
|------|--------|------|
| 主要目的 | 产生**值** | 产生**副作用**（I/O、改状态） |

新增 **`Stmt`** AST 层次（可扩展 `GenerateAst` 或手写）：

| 语句 | 形式 |
|------|------|
| **Expression stmt** | `expr;` |
| **Print** | `print expr;` |

- Parser 解析 **statement 列表**。
- Interpreter 实现 **`Stmt.Visitor`**（与 `Expr.Visitor` 并列）。

---

## §8.2 变量声明（Variable Declarations）

- **`var name = init;`** 或 **`var name;`**（无 init → **`nil`**）。
- 语法上区分 **declaration** vs 普通 **statement**（作用域规则不同）。
- AST：如 **`Stmt.Var`**（name + initializer 表达式）。

---

## §8.3 环境（Environments）

**`Environment`**：核心状态容器。

```text
HashMap<String, Object>  // 变量名 → Lox 值
```

| 操作 | 行为 |
|------|------|
| **define(name, value)** | 在当前环境**定义**（顶层允许重复 define 的语义见书） |
| **get(name)** | **查找**变量值 |

Interpreter 持有 **current Environment**。

**本仓库**：RFR 第 1 章作用域 · 与 Rust 所有权/绑定对比（Lox 为动态 + 链式环境）。

---

## §8.4 赋值（Assignment）

- 语法：`identifier = expression`（**右结合**）。
- 左侧必须是 **l-value**（合法赋值目标），不能是任意表达式。

### 解析技巧

1. 先把左侧按**普通表达式**解析（如 `Variable` 节点）。
2. 若后面有 **`=`** → 验证左侧合法 → 转为 **`Expr.Assign`**（target + value）。

求值：先求 **value**，再写入 Environment。

---

## §8.5 块作用域（Block Scope）

- **`{ ... }`** 创建**局部词法作用域**。
- **Shadowing**：内层 `var x` 可遮蔽外层同名 `x`。

### 环境链（Environment Chain）

```text
inner Environment  ──enclosing──▶  outer Environment  ──▶ … ──▶  global
```

| 时机 | 操作 |
|------|------|
| 进入 block | **新建** Environment，**enclosing** 指向当前环境，推入链 |
| 离开 block | **弹出**，恢复外层 |

- **lookup**：当前 env 没有 → 沿 **enclosing** 向上找。

**clox 预告**：局部变量用**栈槽 + 编译期深度**，不用 Java HashMap（ch22）。

---

## 本章速记

```text
§8.1  Stmt · expr; · print
§8.2  var · 默认 nil
§8.3  Environment + HashMap
§8.4  Assign · 先 parse 再验 l-value
§8.5  块 · 环境链 · shadowing
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **9** | [chapter09 · Control Flow](../chapter09_control-flow/) | `if` / `while` / `for` · 图灵完备 |

---

## 阅读进度

- [x] §8.1～§8.5 结构梳理（本章笔记）
- [ ] 实现 `Stmt` visitor + 嵌套块 shadowing 测试
- [ ] 本章 Challenges
