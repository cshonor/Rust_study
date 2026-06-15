# 第 18 章 · Types of Values（值的类型）

> 在线：[types-of-values.html](https://craftinginterpreters.com/types-of-values.html) · 中文：[第 18 章 值的类型](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch17 Compiling](../chapter17_compiling-expressions/17-compiling-expressions.md)

---

## 本章定位

ch15～17 的 VM 本质是 **Unityped（仅数字）**；本章引入 **动态类型**：`number` · `bool` · `nil`，并为后续字符串、对象打基础。

| 对比 | ch15 Value | ch18 Value |
|------|------------|------------|
| 表示 | 裸 **`double`** | **Tagged union** |
| 字面量 | 数字 | **`true` / `false` / `nil`** |
| 运算 | 纯算术 | **truthy** · 比较 · 类型检查 |

| 主题 | 要点 |
|------|------|
| **Tagged Union** | `type` 标签 + **`union`** 载荷 |
| **Falsiness** | 仅 `nil`、`false` 为假 |
| **Equality / Compare** | `==` … · **`<=` 脱糖** |
| **Runtime Errors** | 算术前 **`checkNumber`** |

---

## 标记联合（Tagged Unions）

C 中用统一 **`Value`** 表示 Lox 任意值：

```c
typedef struct {
  ValueType type;
  union {
    double number;
    bool boolean;
  } as;
} Value;
```

| 宏族 | 用途 |
|------|------|
| **`IS_NUMBER(v)`** / **`IS_BOOL`** / **`IS_NIL`** | 类型判别 |
| **`AS_NUMBER(v)`** 等 | 解包（前提已检查） |
| **`NUMBER_VAL(x)`** / **`BOOL_VAL`** / **`NIL_VAL`** | 构造 Value |

**`ValueType` 枚举**：`VAL_nil` · `VAL_bool` · `VAL_number`（字符串/对象类型在 ch19+ 扩展）。

**对照 jlox [ch7](../../part02_jlox/chapter07_evaluating-expressions/07-evaluating-expressions.md)**：Java **`Object`** 装箱；clox **显式 tag**，无 GC 压力于小值。

**注**：部分 VM 用 **NaN boxing** 把 tag 塞进 double 位型；本书 ch18 先用 **清晰 struct**，易读易 debug（与目录「NaN tagging」为后续优化方向，非本章必实现）。

---

## 真假值与逻辑非（Falsiness and Logical Not）

**Lox 规则**（与 [ch3 规格](../../part01_welcome/chapter03_the-lox-language/03-the-lox-language.md) 一致）：

| 值 | Truthy? |
|----|:-------:|
| **`nil`** | 假 |
| **`false`** | 假 |
| 其他（含 **`0`**、**`""`** 将来） | **真** |

**编译 / 指令**：

- 字面量 **`true`/`false`/`nil`** → **`emitConstant(BOOL_VAL(...))`** 等。
- **`!expr`**：compile `expr` → **`OP_NOT`**（VM 按 falsiness 压 `true/false`）。

---

## 相等与比较（Equality and Comparison）

新增 opcode（示意）：

| 源码 | 字节码思路 |
|------|------------|
| **`==` / `!=`** | **`OP_EQUAL`** · **`OP_NOT`**（不等） |
| **`<` / `>`** | **`OP_GREATER`** 等（约定操作数顺序） |
| **`<=` / `>=`** | **编译期脱糖**，减 opcode 数量 |

**`a <= b` 脱糖**：

```text
compile a
compile b
OP_GREATER      // 等价 a > b
OP_NOT          // !(a > b) 即 a <= b
```

| 优点 | 说明 |
|------|------|
| VM 更小 | 少实现 **`OP_LESS_EQUAL`** 等 |
| 经典技巧 | 与 ch9 **`for` 脱糖** 同属「前端改写」 |

**比较类型**：Lox 允许 **`==`** 跨类型；**`<`** 等要求 **number**（运行时检查）。

---

## 运行时错误（Runtime Errors）

多类型后，**`-false`**、**非 number 相加** 等需拦截。

| 机制 | 说明 |
|------|------|
| **`checkNumber(value)`** | 非 number → **`runtimeError("Operands must be numbers.")`** |
| **行号** | 用 ch14 **`lines[ip]`** |
| **中止** | **`INTERPRET_RUNTIME_ERROR`** · 不崩溃进程 |

**对照 jlox**：自定义 **`RuntimeError`** 异常 + 行号；clox **C 函数 + 返回码**。

**栈上类型错误**：在 **`OP_ADD`** 等执行前检查 **两个 pop 值**。

---

## 本章速记

```text
Value     type tag + union · IS_/AS_/_VAL 宏
Falsiness nil/false 假 · OP_NOT
Compare   <= >= 脱糖为 GREATER + NOT
Runtime   checkNumber · runtimeError + line
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **19** | [chapter19 · Strings](../chapter19_strings/) | **`Obj`** · **`ObjString`** |
| **20** | Hash Tables | 开放寻址 · **intern** |
| **22** | Local Variables | 局部 slot · 非仅全局 |

---

## 自测

1. 为什么 `0` 在 Lox 里是 truthy？
2. `<=` 脱糖后 VM 需要几条指令？比单独 `OP_LESS_EQUAL` 多什么？
3. Tagged union 与 jlox 的 `Object` 各适合什么运行时？

---

## 阅读进度

- [x] Tagged union / falsiness / 比较 / 运行时错误 结构梳理（本章笔记）
- [ ] 列出 `ValueType` 扩展路径（string/obj）
- [ ] 本章 Challenges
