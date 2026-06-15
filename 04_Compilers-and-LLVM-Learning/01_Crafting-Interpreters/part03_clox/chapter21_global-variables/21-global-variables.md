# 第 21 章 · Global Variables（全局变量）

> 在线：[global-variables.html](https://craftinginterpreters.com/global-variables.html) · 中文：[第 21 章 全局变量](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch20 Hash Tables](../chapter20_hash-tables/20-hash-tables.md)

---

## 本章定位

**ch20 哈希表** 落地为 **`vm.globals`** → Lox 终于有了 **状态**。同时引入 **语句**、**`print`**、**`var`**，以及 Pratt 中的 **赋值优先级 / `canAssign`**。

| 对比 | jlox ch8 | clox ch21 |
|------|----------|-----------|
| 存储 | **`Environment` HashMap** | **`Table vm.globals`** |
| 声明 | **`Stmt.Var` + define** | **`OP_DEFINE_GLOBAL`** |
| 读取/写 | **`get` / `assign` + distance** | **`OP_GET/SET_GLOBAL`**（暂仅全局） |
| 表达式语句 | Visitor | compile 后 **`OP_POP`** |

| 主题 | 要点 |
|------|------|
| **Statements** | 栈效应为 0 · **`OP_POP`** |
| **`var` 声明** | **`OP_DEFINE_GLOBAL`** |
| **读/写** | **`OP_GET/SET_GLOBAL`** · 未定义报错 |
| **Assignment** | **`canAssign`** · 非法 l-value |

---

## 语句与状态效果（Statements）

| 语句 | 编译概要 |
|------|----------|
| **`print expr;`** | compile `expr` → **`OP_PRINT`** |
| **`expr;`** | compile `expr` → **`OP_POP`**（丢弃栈顶） |

**栈效应为 0**：

- 语句执行后 **栈深度与进入前相同**。
- 表达式求值会 **push 结果**；纯副作用语句 **不应留垃圾**。

```text
expression statement:
  ... compile expr ...   // stack: [..., result]
  OP_POP                 // stack: [...]
```

**对照 jlox [ch8](../../part02_jlox/chapter08_statements-and-state/08-statements-and-state.md)**：表达式语句 Visitor 直接 **忽略返回值**；clox 显式 **POP**。

---

## 全局变量声明（Variable Declarations）

```lox
var breakfast = "bagels";
var lunch;    // → nil
```

**编译 `var name = init;`**：

1. 若有 init → compile initializer；否则 **`OP_NIL`**（或 constant nil）。
2. 变量名 lexeme → **`identifierConstant`** → **intern `ObjString*`** → 常量池索引。
3. **`OP_DEFINE_GLOBAL`** + 名字常量索引。

**VM 执行 `OP_DEFINE_GLOBAL`**：

```text
name = readString(constant)
value = pop()
tableSet(&vm.globals, name, value)
```

**顶层重复 `var`**：允许（与 jlox 一致）→ **覆盖** 全局绑定。

---

## 读取和赋值（Reading and Assignment）

**标识符作为表达式**（读取）：

```text
OP_GET_GLOBAL  name_index
→ tableGet(&vm.globals, name) → push(value)
  失败 → runtimeError("Undefined variable '...'.")
```

**赋值 `name = expr`**：

```text
compile expr          // 先算右值
OP_SET_GLOBAL  name   // pop 写入表 · 再 push（表达式值=赋值结果）
```

| Opcode | 作用 |
|--------|------|
| **`OP_DEFINE_GLOBAL`** | 声明：pop 初值 · **新建** 绑定 |
| **`OP_GET_GLOBAL`** | 读：push 值 |
| **`OP_SET_GLOBAL`** | 写：pop 赋值 · push 同一值 |

**常量池存名字**：每条指令只带 **1 字节索引** → 名字字符串 **intern 一次**。

---

## 赋值的优先级（Assignment Precedence）

**`=` 是表达式**，但左值必须是 **可赋值目标**（变量；后续 field 等）。

**非法**：`a + b = c` · `1 = 2`

**Pratt 机制：`canAssign` 标志**

| 传递 | 含义 |
|------|------|
| **`parsePrecedence(PREC_ASSIGNMENT)`** | 顶层允许赋值 |
| 更高优先级 infix（如 **`+`**）调用子 parse 时传 **`canAssign = false`** |
| **`variable()` prefix** | 读变量；若 **`canAssign && match(EQUAL)`** → 生成 **SET** 而非 GET |

```text
parsePrecedence(ASSIGNMENT):
  canAssign = true  →  identifier 后可吃 '='

parsePrecedence(ADDITION):  // 在 + 的 infix 里
  compile left
  compile right with canAssign=false
  → 右侧不会出现非法 a = ...
```

**对照 jlox ch8**：Parser 先 parse 表达式再 **回溯** 成 `Assign`；clox **编译期** 用 **`canAssign`** 约束，**无 AST 回溯**。

---

## 全局变量管线小结

```text
var x = 1;
  CONSTANT 1
  CONSTANT "x"    // interned name
  DEFINE_GLOBAL

print x;
  GET_GLOBAL "x"
  PRINT

x = 2;
  CONSTANT 2
  SET_GLOBAL "x"
```

---

## 本章速记

```text
语句    表达式 stmt 末尾 OP_POP · print → OP_PRINT
声明    OP_DEFINE_GLOBAL · 名入常量池 intern
读写    OP_GET/SET_GLOBAL · vm.globals 表
赋值    canAssign 控制 · 仅 ASSIGNMENT 级可 =
错误    未定义全局 · runtimeError + line
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **22** | [chapter22 · Local Variables](../chapter22_local-variables/) | 栈 slot · **`OP_GET/SET_LOCAL`** |
| **23** | Jumping Back and Forth | 控制流 bytecode |
| **11** jlox | Resolver | distance · 闭包静态绑定（clox ch22+ 对应） |

---

## 自测

1. 为什么 `SET_GLOBAL` 执行后还要把值 push 回栈？
2. `canAssign=false` 时遇到 `foo = 1` 会怎样？
3. 全局变量名为何放在常量池而不是指令里嵌字符串？

---

## 阅读进度

- [x] 语句 / 全局 var / GET·SET / canAssign 结构梳理（本章笔记）
- [ ] 对照 jlox ch8 Environment 与 clox globals 表
- [ ] 本章 Challenges
