# 第 9 章 · Control Flow（控制流）

> 在线：[control-flow.html](https://craftinginterpreters.com/control-flow.html) · 中文：[第 9 章 控制流](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch08 Statements](../chapter08_statements-and-state/notes/08-statements-and-state.md)

---

## 本章定位

加入 **条件分支 + 循环** → Lox 解释器在此达到 **图灵完备**（可表达通用计算）。

| 小节 | 主题 |
|------|------|
| **§9.2** | `if/else` · 悬挂 else · 贪婪匹配 |
| **§9.3** | `and` / `or` · 短路 · 返回操作数本身 |
| **§9.4** | `while` |
| **§9.5** | `for` · **脱糖** 为 `while` + block |

（原文 **§9.1** 多为控制流引言 / 图灵完备背景，与 §9.2 连贯阅读即可。）

---

## §9.2 条件执行（If Statements）

- 实现 **`if (cond) … else …`**。
- **`Stmt.If`**：condition + thenBranch + optional elseBranch。

### 悬挂 else（Dangling else）

```lox
if (first) if (second) whenTrue; else whenFalse;
```

`else` 应绑定**最近的未闭合 `if`**（`second`），而非 `first`。

**解析策略**：**贪婪匹配** —— 见到 `else` 总是挂到**刚解析完的内层 `if`** 上（递归下降自然实现）。

**执行**：求 condition 的 **truthy** → 执行 then 或 else 分支（Stmt visitor）。

---

## §9.3 逻辑操作符（Logical Operators）

`and` / `or` 在 Lox 中：

| 特性 | 说明 |
|------|------|
| **控制流味道** | 不仅是算术式二元运算 |
| **短路（Short-circuit）** | 左操作数已能定结果 → **不求值**右操作数 |
| **返回值** | 返回**决定结果的那个操作数的原值**，不一定是 `true/false` |

示例：

```lox
"hi" or 2    // → "hi"（左 truthy，右不执行）
nil and foo  // → nil（左 falsy，右不执行）
```

**实现位置**：

- **Parser**：`and` / `or` 优先级低于 equality（ch6 分层已预留）或单独逻辑层。
- **Interpreter**：`visitLogicalExpr` 中按短路规则求值，**不**一律返回 Boolean。

**对照 ch3 §3.4**：表达式层的短路 · 与 C/Java `&&`/`||` 返回 boolean 的差异。

---

## §9.4 While 循环（While Loops）

- 语法：`while (cond) body;`
- **`Stmt.While`**：condition + body。
- **执行**：Java **`while`** 循环 —— 反复 truthy 检测 + 执行 body。

**clox**：ch23 用**字节码跳转**实现，而非 Java loop。

---

## §9.5 For 循环（For Loops）

C 风格三件套：

```lox
for (init; cond; increment) {
  body
}
```

### 脱糖（Desugaring）

**后端不必实现专用 `for` 执行逻辑** —— Parser 在前端把 `for` **翻译成**已有 AST：

```text
{
  init;
  while (cond) {
    body;
    increment;
  }
}
```

| 优点 | 说明 |
|------|------|
| 解释器更简单 | 只实现 `while` + block |
| 经典编译器技巧 | 语法糖在前端消掉 |

**本仓库**：Rust `for` desugar 到 loop/iterator · 同一「前端脱糖」思想。

---

## 三阶段能力小结（ch7～9）

```text
ch7  表达式求值        「计算器」
ch8  语句 + 变量 + 环境  「有记忆」
ch9  if / while / for    「图灵完备」
```

---

## 本章速记

```text
§9.2  if/else · dangling else · 贪婪绑定
§9.3  and/or 短路 · 返回操作数值非必 bool
§9.4  while · Stmt.While
§9.5  for 脱糖为 block + while
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **10** | [chapter10 · Functions](../chapter10_functions/) | **`fun`** · 一等函数 |
| **11** | Resolving and Binding | 闭包 · **Resolver** |

---

## 阅读进度

- [x] §9.2～§9.5 结构梳理（本章笔记）
- [ ] 测试悬挂 else、短路、`for` 脱糖后的 AST
- [ ] 本章 Challenges
