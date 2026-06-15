# 第 17 章 · Compiling Expressions（编译表达式）

> 在线：[compiling-expressions.html](https://craftinginterpreters.com/compiling-expressions.html) · 中文：[第 17 章 编译表达式](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch16 Scanning](../chapter16_scanning-on-demand/16-scanning-on-demand.md)

---

## 本章定位

**前端扫描器（ch16）与后端 VM（ch15）正式接合**：编写 **字节码编译器**，把源码 **直接编译成 Chunk**，不建 AST。

| 对比 | jlox | clox |
|------|------|------|
| 中间表示 | **AST** | **无 AST** |
| 趟数 | 解析 → 执行（多趟） | **单遍**：边 parse 边 emit |
| 表达式解析 | 递归下降（ch6） | **Pratt 解析器** |

| 主题 | 要点 |
|------|------|
| **单遍编译** | parse + emit 同步 |
| **Pratt Parser** | 前缀/中缀分发表 · 优先级 |
| **Emitting** | 字面量 · 括号 · 一元/二元算术 |
| **错误恢复** | **`panicMode`** · 同步到语句边界 |

---

## 单遍编译（Single-Pass Compilation）

```text
Source → Scanner (按需 Token)
           ↓
      Compiler: Pratt parse ──emit──► Chunk (bytecode)
           ↓
      VM.run()  (ch15)
```

| 特点 | 说明 |
|------|------|
| **不分配 AST 节点** | 省内存、少指针追逐 |
| **编译即解析** | 每识别一个语法结构，立刻 **`emitByte`** |
| **对照 ch2** | 传统编译器前端 → **IR（此处为 bytecode）** 单趟 |

**jlox 对照**：Parser 产出 `Expr` 树 → Interpreter Visitor；clox **跳过树**，指令流即程序。

---

## 普拉特解析器（A Pratt Parser）

Vaughan Pratt 的 **自顶向下运算符优先级解析**（Top-Down Operator Precedence）。

**核心结构**（C 中函数指针分发表）：

```c
typedef void (*ParseFn)(bool canAssign);

ParseRule rules[] = {
  // TokenType → { prefixFn, infixFn, precedence }
};
```

| 概念 | 说明 |
|------|------|
| **prefix 解析函数** | 处理 **`−`**、**`(`**、数字等 **前缀** 位置 |
| **infix 解析函数** | 处理 **`+`**、**`*`** 等 **中缀** 位置 |
| **precedence** | 数值越大绑定越紧；控制 **结合性**（左结合：先 parse 高优先级右侧） |

**入口**：

```c
void expression() {
  parsePrecedence(PREC_ASSIGNMENT);
}
```

**`parsePrecedence(minPrec)`** 循环：

1. 读 prefix rule → 调用 prefix 函数（已消费首 token）。
2. while 下一 token 的 infix 优先级 ≥ `minPrec` → 调用 infix 函数。

**对照 jlox [ch6](../../part02_jlox/chapter06_parsing-expressions/06-parsing-expressions.md)**：递归下降每层一个函数；Pratt 用 **表驱动 + 优先级** 统一处理运算符。

---

## 发出字节码（Emitting Bytecode）

**编译器辅助**：

| API | 作用 |
|-----|------|
| **`emitByte(op)`** | 写 opcode（+ 行号） |
| **`emitConstant(value)`** | 常量入池 → **`OP_CONSTANT` idx** |
| **`emitReturn()`** | 结束片段 |

**典型 lowering**（栈 VM 顺序）：

| 源码 | 编译策略 |
|------|----------|
| 数字 `1.2` | `emitConstant(1.2)` |
| `(expr)` | 编译 `expr`（括号不 emit） |
| **`-x`** | 先 **`compile x`**（push）→ **`OP_NEGATE`** |
| **`a + b`** | compile `a` → compile `b` → **`OP_ADD`** |

**关键**：一元 `-` **先子后父**——操作数先入栈，再 negate，与 ch15 栈式语义一致。

**分组 `(`**：prefix 函数 **`(`** 内调 `expression()`，再 **`consume(RIGHT_PAREN)`**。

---

## 语法错误处理（Handling Syntax Errors）

C **无异常** → 编译错误用 **标志 + 同步** 恢复。

| 机制 | 说明 |
|------|------|
| **`hadError`** | 本次 compile 是否失败 |
| **`panicMode`** | 已报错 → **抑制级联错误** |
| **`error(message)`** | 打印行号 + 消息；置位上述标志 |
| **`synchronize()`** | 恐慌模式下 **跳到语句边界**（`;`、`}`、class/fun/for/if/var/while 等） |

```text
语法错误 → panicMode = true
  → 后续 error 被忽略
  → synchronize() 找到安全同步点
  → panicMode = false，继续解析（尽量多报独立错误）
```

**对照 jlox**：Java **`ParseError`** + throw；clox **不 longjmp**，靠状态机恢复。

---

## 编译管线（本章结束时）

```text
compile(source):
  initScanner(source)
  // Pratt + emit
  emitReturn()
  if (!hadError) interpret(&chunk)
```

---

## 本章速记

```text
单遍    无 AST · parse 同时 emit Chunk
Pratt   rules[] · prefix/infix · precedence
Emit    先 compile 子表达式再 OP_* · 一元 − 后 NEGATE
错误    panicMode + synchronize 到语句边界
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **18** | [chapter18 · Types of Values](../chapter18_types-of-values/) | **`Value`**  tagged union · bool/nil |
| **19** | Strings | **`ObjString`** · 堆对象 |
| **21** | Global Variables | **`canAssign`** · 全局变量 opcode |

---

## 自测

1. Pratt 与 jlox 递归下降相比，换运算符优先级时改哪里？
2. 为什么 `-123` 的编译顺序是 `CONSTANT 123` 然后 `OP_NEGATE`？
3. `panicMode` 存在的意义是什么——若没有会怎样？

---

## 阅读进度

- [x] 单遍 / Pratt / Emit / 错误恢复 结构梳理（本章笔记）
- [ ] 对照 `rules[]` 列出各 Token 的 prefix/infix
- [ ] 本章 Challenges
