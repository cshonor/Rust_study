# 第 11 章 · Resolving and Binding（解析与绑定）

> 在线：[resolving-and-binding.html](https://craftinginterpreters.com/resolving-and-binding.html) · 中文：[第 11 章 解析和绑定](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch10 Functions](../chapter10_functions/notes/10-functions.md)

---

## 本章定位

ch10 闭包靠 **环境链 + 按名向上查找** 能跑通多数情况，但存在 **词法作用域 Bug**（外层重新 `var` 后，闭包可能绑错变量）。ch11 引入 **语义分析 (Semantic Analysis)**：**运行前** 解析每个变量「在第几层环境」，运行时 **按 distance 直取**。

| 阶段 | ch4～10 | ch11 新增 |
|------|---------|-----------|
| 扫描 | Token | — |
| 解析 | AST | — |
| **分析** | — | **`Resolver`** 遍历 AST，写 binding |
| 执行 | Interpreter | **`getAt(distance)` / `assignAt(distance)`** |

| 小节 | 主题 |
|------|------|
| **§11.1** | **静态 / 词法作用域** · 动态查找缺陷 |
| **§11.2** | **语义分析** · 独立于执行 |
| **§11.3** | **`Resolver`** · 作用域栈 |
| **§11.4** | 解释已解析变量 · **distance** |
| **§11.5** | **Resolution Errors** · 如顶层 `return` |

---

## §11.1 静态作用域（Static Scope）

**Lox 规则**：变量归属由**源码书写位置（词法结构）**决定，而非运行时动态栈。

**ch10 动态查找的问题**（示意）：

```lox
var a = "global";
{
  fun show() { print a; }
  var a = "block";
  show();  // 期望 "global" 还是 "block"？
}
```

- 若 `show` 闭包捕获的是 **Environment 引用**，块内 **`var a`** 可能在同一链上 **遮蔽** 全局 `a`，闭包读到 **block**。
- **词法作用域**要求：闭包内的 `a` 应绑定到 **定义 `show` 时可见的那个 `a`（global）**。

| 概念 | 说明 |
|------|------|
| **Lexical scoping** | 编译 / 分析阶段确定绑定 |
| **Dynamic scoping** | 运行时按调用栈找（Lox **不是**） |

---

## §11.2 语义分析（Semantic Analysis）

**办法**：在 **AST 构建完成之后、interpret 之前**，增加一趟 **只分析变量绑定** 的遍历。

```text
Source → Scanner → Parser → AST
                              ↓
                         Resolver（语义分析）
                              ↓
                         Interpreter
```

| 特点 | 说明 |
|------|------|
| 不执行用户代码 | 只 walk 树，维护作用域结构 |
| 输出 | 每个 **`Expr.Variable` / `Expr.Assign`** 关联 **栈深度 distance** |
| 对照 ch2 | 编译之山上的 **Analysis** 阶段 |

---

## §11.3 解析器类（A Resolver Class）

**`Resolver`** 实现 **`Expr.Visitor`** + **`Stmt.Visitor`**（与 Interpreter 类似，但**不求值**）。

**核心结构**：**作用域栈**

```text
scopes: Stack<Map<String, VariableState>>
  push  进入 block / function
  pop   离开
  declare(name)   当前层标记「已声明」
  define(name)    当前层标记「已定义」
```

| 变量状态 | 含义 |
|----------|------|
| **Declared** | `var x` 已解析，尚未执行到 initializer |
| **Defined** | 已可读写 |

**遍历要点**：

- **`Stmt.Var`**：`declare` → 递归 initializer → `define`。
- **`Stmt.Function`**：新 scope → declare 函数名 → 解析参数与 body → `define` 函数名。
- **`Expr.Variable` / `Assign`**：在当前栈中 **resolve**，计算 **distance**（向外几层 `enclosing`）。

**存储 binding**：Interpreter 侧用 **`Map<Expr, Integer> locals`**（或类似）记录每个变量表达式节点的 **distance**。

---

## §11.4 解释已解析的变量（Interpreting Resolved Variables）

运行时不再 **`environment.get(name)` 逐层向上搜**：

| API | 行为 |
|-----|------|
| **`getAt(distance, name)`** | 从当前 env 沿 `enclosing` 走 **distance** 步，再 **get** |
| **`assignAt(distance, name, value)`** | 同上，再 **assign** |

**`visitVariableExpr`**：

```text
if (locals.containsKey(expr))
  return environment.getAt(locals.get(expr), name);
else
  return globals.get(name);  // 顶层全局
```

**效果**：

- 闭包内变量 **固定** 到定义时那层 binding。
- 内层 **`var` 遮蔽** 不影响已解析的 **distance**。
- 性能：O(1) 直达，非 O(深度) 链式查找。

---

## §11.5 解析错误（Resolution Errors）

静态扫描可做的 **compile-time（解析期）检查**：

| 规则 | 示例 |
|------|------|
| **`return` 只能在函数内** | 顶层 `return 1;` → **Resolver 报错** |
| 重复参数名 | `fun f(a, a)` |
| （后续 ch12）**`this` 只能在方法内** | 类外 `this` |

与 ch4 **语法错误** 区分：AST 合法，但 **语义不合法**。

**管线**：

```text
Lox.run:
  parse → resolve → interpret
          ↑ 失败则打印错误，不执行
```

---

## Resolver 与 Interpreter 分工

```text
        Resolver                    Interpreter
        ────────                    ───────────
输入    AST                         AST + locals map
关心    作用域嵌套、声明顺序         值、控制流、调用
输出    locals[expr]=distance       副作用 + 返回值
```

---

## 本章速记

```text
§11.1  词法作用域 · 动态查找闭包 Bug
§11.2  AST 后、运行前的语义分析趟
§11.3  Resolver + 作用域栈 declare/define
§11.4  getAt/assignAt(distance)
§11.5  顶层 return 等 resolution error
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **12** | [chapter12 · Classes](../chapter12_classes/) | **`class`** · 实例 · **`this`** |
| **13** | Inheritance | **`super`** · 继承 |

**clox 预告**：Part III 同样在编译期做 **Upvalue / 局部 slot** 绑定，思想与 Resolver 同源。

---

## 自测

1. distance = 0 表示变量在哪一层环境？
2. 为什么 `Resolver` 访问 `Expr.Literal` 可以是空操作？
3. 全局变量为何可以不进 `locals` map？

---

## 阅读进度

- [x] §11.1～§11.5 结构梳理（本章笔记）
- [ ] 手工 trace 一段嵌套 `var` + 闭包的 resolve 过程
- [ ] 本章 Challenges
