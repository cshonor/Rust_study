# 第 10 章 · Functions（函数）

> 在线：[functions.html](https://craftinginterpreters.com/functions.html) · 中文：[第 10 章 函数](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch09 Control Flow](../chapter09_control-flow/notes/09-control-flow.md)

---

## 本章定位

ch9 已图灵完备；ch10 加入 **函数** → 代码复用、参数传递、**闭包**（§10.6 初版，ch11 修 Bug）。

| 概念 | ch8～9 | ch10 |
|------|--------|------|
| 可执行单元 | 全局语句 + 块 | **`fun`** 声明 · **`()` 调用** |
| 环境 | 单层 / 链式 lookup | **每次调用新建 Environment** |
| 一等公民 | 变量存值 | **函数也是值**（`LoxFunction`） |

| 小节 | 主题 |
|------|------|
| **§10.1** | 函数调用 · `()` 后缀 · **Arity** · 最多 255 参数 |
| **§10.2** | **Native** 函数 · `clock()` |
| **§10.3** | `fun` 声明解析 |
| **§10.4** | **`LoxFunction`** · 调用时隔离环境 |
| **§10.5** | **`return`** · 用异常跳出递归栈 |
| **§10.6** | 嵌套函数 · **闭包** · 捕获词法环境 |

---

## §10.1 函数调用（Function Calls）

- 语法：`callee(arg1, arg2, …)`。
- **Parser**：把 **`()` 视为极高优先级的后缀运算符**（与 ch6 一元/二元分层衔接，call 在 call/column 层）。
- **Arity**：实参与形参个数必须一致，否则 **RuntimeError**。
- **255 参数上限**：与后续 **clox** 字节码格式对齐（单条指令操作数宽度），jlox 提前遵守。

**运行时抽象**：

| 接口 / 类 | 作用 |
|-----------|------|
| **`LoxCallable`** | 统一「可调用对象」：`call(Interpreter, List<Object> args)` + `arity()` |
| 校验 | 调用前比对 `args.size()` 与 `arity()` |

**AST**：`Expr.Call` —— callee 表达式 + arguments 列表。

---

## §10.2 原生函数（Native Functions）

Lox 需要与宿主（Java）交互 → **内置原生函数**，由 Java 实现、注册到全局环境。

| 要点 | 说明 |
|------|------|
| 实现方式 | 匿名类实现 **`LoxCallable`**（或 lambda） |
| 本书唯一内置 | **`clock()`** → 返回秒级运行时间（`System.currentTimeMillis()` 等） |
| 用途 | 后续性能对比、基准测试 |

全局 **`Environment`** 中 `define("clock", …)`，与普通 `LoxFunction` 同样通过 **`LoxCallable`** 调用。

---

## §10.3 函数声明（Function Declarations）

```lox
fun name(param1, param2) {
  // body
}
```

- **`fun`** 关键字 → 函数名 · 参数列表 · **`{}` 块** 为 body。
- **AST**：`Stmt.Function`（name, params, body block）。
- **执行**：在**当前环境**中 `define(name, LoxFunction)`，函数名与变量一样可被后续引用。

---

## §10.4 函数对象（Function Objects）

**`LoxFunction`** 表示 Lox 中的一等函数值：

| 字段 / 行为 | 说明 |
|-------------|------|
| 闭包环境 | 声明时所在的 **`Environment`**（§10.6 完善） |
| `call()` | **新建子 Environment**，`enclosing` 指向闭包环境 |
| 参数 | 在子环境中 **define** 每个形参 |
| body | 在新环境中执行 block |

**为何每次调用新建环境？**

- 递归 / 多次调用时，局部变量与参数**互不干扰**。
- 对照 ch8 **环境链**：函数体 = 新的块作用域，但 **enclosing 是定义处环境**（闭包），不是调用处。

```text
call foo():
  new env(enclosing = foo.closure)
    define params...
    execute body
```

---

## §10.5 返回语句（Return Statements）

- 语法：`return expr;` 或 `return;`（等价 `return nil;`）。
- **AST**：`Stmt.Return`（可选 value）。

**问题**：jlox 用 **Visitor 递归** 执行语句，嵌套很深时，用普通 `return` 只能跳出当前 Java 方法，无法「从任意深度立刻结束函数体」。

**解法**：**异常作为非本地控制流**

| 类 | 作用 |
|----|------|
| **`Return`**（自定义异常） | 携带返回值 `Object value` |
| 执行 `return` | `throw new Return(value)` |
| `LoxFunction.call()` | `try { execute body } catch (Return r) { return r.value; }` |

经典技巧：树遍历解释器里用异常模拟 **longjmp** / 结构化退出。

---

## §10.6 局部函数与闭包（Local Functions and Closures）

Lox 允许 **函数内定义函数**：

```lox
fun outer() {
  var x = "local";
  fun inner() {
    print x;  // 闭包：读 outer 的 x
  }
  return inner;
}
```

**闭包 (Closure)**：函数 + **其定义时的词法环境**。

| 机制 | 说明 |
|------|------|
| **`LoxFunction` 保存 `closure`** | 创建时记录当前 `Environment` |
| 调用 `inner` | 新环境的 `enclosing` = 保存的 closure，而非调用栈外层 |
| 结果 | 外层局部变量在 inner 执行时仍可见 |

**注意**：此章的「链式按名查找」在 **外层变量被重新声明** 时会有 Bug → **ch11 Resolver** 修复。

**对照**：

- **ch2** 语义分析阶段 · **ch11** 静态绑定 distance
- **Rust** 闭包捕获 · **LLVM** 仍用环境/帧，思路同源

---

## 三阶段能力小结（ch8～10）

```text
ch8   变量 + Environment
ch9   控制流
ch10  fun / call / return / 闭包（初版）
ch11  Resolver → 闭包 Bug 修复
```

---

## 本章速记

```text
§10.1  Call 后缀 · LoxCallable · arity · ≤255 args
§10.2  Native · clock()
§10.3  Stmt.Function · fun 解析
§10.4  LoxFunction · 每次 call 新 Environment
§10.5  Return 异常跳出 Visitor 栈
§10.6  closure 捕获定义时环境
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **11** | [chapter11 · Resolving](../chapter11_resolving-and-binding/) | **Resolver** · 静态作用域 · distance |
| **12** | [chapter12 · Classes](../chapter12_classes/) | `class` · `this` · OOP |

---

## 自测

1. 为什么 `()` 解析成「后缀运算符」而不是独立语句？
2. `return` 为什么用异常而不是改 Visitor 返回值类型？
3. 闭包里的 `enclosing` 指向哪里——定义处还是调用处？

---

## 阅读进度

- [x] §10.1～§10.6 结构梳理（本章笔记）
- [ ] 实现 / 对照 jlox 源码：`LoxFunction.call`、`Return`
- [ ] 本章 Challenges
