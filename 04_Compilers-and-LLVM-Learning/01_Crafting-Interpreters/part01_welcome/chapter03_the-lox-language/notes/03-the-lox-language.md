# 第 3 章 · The Lox Language（Lox 语言概览）

> 在线：[the-lox-language.html](https://craftinginterpreters.com/the-lox-language.html) · 中文：[第 3 章 Lox 语言](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part I · [part01_welcome](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch02 领域地图](../chapter02_map-of-the-territory/notes/02-map-of-the-territory.md)

---

## 本章定位

Part I 最后一章：**在写解释器之前**，完整介绍 **Lox 的语法与语义**——友好、平缓，不立刻陷入实现细节。读完 = 拿到接下来 **jlox / clox 要实现的目标蓝图**。

| 小节（原文结构） | 主题 |
|------------------|------|
| **§3.1** Hello, Lox | C 系语法，降低学习成本 |
| **§3.2** A High-Level Language | 动态类型 + 自动内存管理（GC） |
| **§3.3～3.9** | 类型、表达式、语句、变量、控制流、函数/闭包、类/继承 |
| **§3.10** The Standard Library | 极简：`clock()` 唯一内置 |

**附录 A1** 有 Lox 完整语法（BNF）；写 parser 时可对照 [`backmatter/appendix01_lox-grammar/`](../../../backmatter/appendix01_lox-grammar/)。

---

## §3.1 你好，Lox（Hello, Lox）

- Lox 语法属于 **C 语言家族**（类似 C、Java、JavaScript）。
- 作者选型理由：读者通常已熟悉 → **减少学新语法**的负担，把精力留给解释器机制。

**与 Rust 对比**：Rust 是静态类型、所有权；Lox 故意走「脚本语言」路线，便于 Part II 先讲语义。

---

## §3.2 一种高级语言（A High-Level Language）

Lox 定位接近 **JavaScript、Scheme、Lua** 等高级脚本语言。

### §3.2.1 动态类型（Dynamic typing）

| 项目 | 说明 |
|------|------|
| **含义** | 变量可存**任意类型**的值 |
| **检查时机** | 类型检查推迟到 **Runtime** |
| **实现影响** | jlox/clox 需运行时类型判别；clox 用 tagged value（**ch18**） |

**本仓库**：RFR 第 2 章静态类型与 layout · 与 Lox 动态模型对照读。

### §3.2.2 自动内存管理（Automatic memory management）

- 不手动 `malloc/free`。
- 依赖 **垃圾回收（GC）** 回收不再使用的内存。
- **jlox**：借 Java GC；**clox**：**ch26** 自实现 GC。

**本仓库**：RFR 第 1 章内存 · 第 9～10 章 · **Nomicon** 有效性。

---

## §3.3 数据类型（Data Types）

Lox 内置类型**精简**：

| 类型 | 说明 |
|------|------|
| **Boolean** | `true` / `false` |
| **Number** | 浮点（书中实现为 double） |
| **String** | 字符串字面量 |
| **Nil** | 「无值」，类似 `null` |

无数组、无字典（标准库也极简）——聚焦语言核心。

---

## §3.4 表达式（Expressions）

| 类别 | 要点 |
|------|------|
| 算术 / 比较 | 常规中缀表达式 |
| 逻辑 | `!`、`and`、`or` |
| **短路求值** | `and` / `or` 具 **short-circuit** → 更像「披着表达式外衣的控制流」 |
| 分组 | `()` 改变运算符优先级 |

**实现预告**：Part II **ch6～7** 解析与求值表达式；clox **ch17** 编译表达式到字节码。

**Rust 对照**：`&&` / `||` 同样短路；但 Rust 中它们是**控制流**而非可嵌在任意表达式位置的「值」（与 Lox 略有不同）。

---

## §3.5 语句（Statements）

| 概念 | 说明 |
|------|------|
| **表达式** | 主要**产生一个值** |
| **语句** | 主要产生**副作用（Effect）**（改状态、I/O 等） |

Lox 语句形态：

| 语句 | 形式 |
|------|------|
| 打印 | `print expr;` |
| 表达式语句 | `expr;` |
| 代码块 | `{ ... }` → 影响**作用域** |

**Design Note（章末）**：*Expressions and Statements* —— 表达式 vs 语句的划分（不计入 30 章正文，与 §3.4～3.5 呼应）。

---

## §3.6 变量（Variables）

- 声明：`var name = init;` 或 `var name;`
- **无初始值**时默认为 **`nil`**
- 作用域：块级 `{ }`（**ch8** 语句与状态 · **ch11** 绑定）

---

## §3.7 控制流（Control Flow）

C 风格，直接沿用：

| 构造 | 用途 |
|------|------|
| `if` / `else` | 条件分支 |
| `while` | 循环 |
| `for` | 循环（语法糖） |

**实现预告**：Part II **ch9** · clox **ch23** Jumping Back and Forth（字节码跳转）。

---

## §3.8 函数（Functions）

| 特性 | 说明 |
|------|------|
| 声明 | `fun name(params) { ... }` |
| 返回 | `return expr;`；无 `return` 到末尾 → 隐式 **`nil`** |
| **一等公民** | 函数可作值：赋给变量、当参数传递 |
| **局部函数** | 函数内可再定义函数 |
| **闭包** | 内部函数**捕获**外部局部变量并保留 |

**实现预告**：Part II **ch10～11** · clox **ch24 Calling and Closures**。

**本仓库**：RFR 闭包与 `Fn` trait · `dyn` 分发 · 与 Lox 闭包语义对照。

---

## §3.9 类（Classes）

**基于类**的 OOP（非 JavaScript 式原型）。

### 声明与实例化

| 项目 | Lox 做法 |
|------|----------|
| 类 | `class Name { ... }` |
| 方法 | 类体中**不写** `fun` |
| 构造实例 | **无 `new`** —— **像调用函数一样调用类** |

### 状态与初始化

- 可**动态**给实例加属性（字段）。
- 方法内 **`this`** → 当前实例。
- **`init()`** 若存在 → 创建实例时**自动**作为构造函数调用。

### 继承（Inheritance）

| 项目 | 说明 |
|------|------|
| 模型 | **单继承** |
| 语法 | `class Child < Parent { ... }` |
| 超类调用 | **`super`** 调用被覆盖的父类方法 |

**实现预告**：Part II **ch12～13** · clox **ch27～28**。

---

## §3.10 标准库（The Standard Library）

- 故意**极简**，把篇幅留给解释器实现。
- **无**字符串 API、**无**文件 I/O 等。
- **唯一内置函数**：**`clock()`** —— 返回自程序启动以来的秒数（供 **ch30 Optimization** 等性能对比）。

---

## Lox 特性 → 本书章节速查

| Lox 特性 | Part II jlox | Part III clox |
|----------|--------------|---------------|
| Token / Scan | ch4 | ch16 |
| Parse / AST | ch5～6 | （编译器内嵌） |
| 表达式 / 语句 | ch7～8 | ch17+ |
| 控制流 | ch9 | ch23 |
| 函数 / 闭包 | ch10～11 | ch24 |
| 类 / 继承 | ch12～13 | ch27～28 |
| GC | （Java） | ch26 |
| `clock()` | 内置绑定 | 内置 + ch30 bench |

---

## 本章速记

```text
C 系语法 · 动态类型 · GC
类型：bool / number / string / nil
表达式有值 · 语句有副作用 · and/or 短路
var · if/while/for · fun 一等 + 闭包
class / this / init · 单继承 < · super
标准库只有 clock()
```

---

## 读后下一步

Part I 读完 → **Part II 开始写代码**

| 章 | 目录 | 内容 |
|:--:|------|------|
| **4** | [chapter04 · Scanning](../../part02_jlox/chapter04_scanning/) | 流水线第一站：**扫描器** → Token |

建议：写 jlox 前扫一眼 **附录 I** 语法；实现中遇歧义回查本章 + A1。

---

## 自测 / 对照（可选）

- [ ] 写 3 行 Lox 代码：含 `var`、闭包、`class` 与 `init` 各一。
- [ ] 解释：`and` 短路为何算「控制流味道」？
- [ ] 对比 Rust：`Lox 动态类型` vs `Rust 静态 + Option` 在**何时**发现类型错误。
- [ ] 说明 Lox **无 `new`** 实例化与 Java/C++ 的差异。

---

## 阅读进度

- [x] §3.1～§3.10 结构梳理（本章笔记）
- [ ] Design Note *Expressions and Statements*
- [ ] 附录 I Lox Grammar（写 ch4 前建议）
- [ ] Part I 完成 → 进入 **ch4 Scanning**
