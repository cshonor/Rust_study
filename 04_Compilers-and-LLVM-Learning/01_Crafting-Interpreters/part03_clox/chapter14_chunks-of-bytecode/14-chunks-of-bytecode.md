# 第 14 章 · Chunks of Bytecode（字节码块）

> 在线：[chunks-of-bytecode.html](https://craftinginterpreters.com/chunks-of-bytecode.html) · 中文：[第 14 章 字节码块](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章（Part II 收官）：[ch13 Inheritance](../../part02_jlox/chapter13_inheritance/13-inheritance.md)

---

## 本章定位

**Part III 奠基**：在造 VM 之前，先定义 **字节码如何存放**。clox 用 C 手写 **紧凑指令序列 + 常量池 + 行号表**，告别 jlox 的 AST 指针树。

| 对比 | jlox (Part II) | clox (Part III) |
|------|----------------|-----------------|
| 程序表示 | **AST** 节点树 | **`Chunk`** 字节数组 |
| 执行 | Visitor 递归遍历 | **VM** 读 ip  dispatch |
| 语言 | Java | **C** |

| 小节 | 主题 |
|------|------|
| **§14.1** | 何为 **Bytecode** · 相对 AST / 机器码 |
| **§14.3** | **`Chunk`** · 动态字节数组 · Opcode |
| **§14.4** | **反汇编** · debug 黑盒 |
| **§14.5** | **常量池** · `OP_CONSTANT` |
| **§14.6** | **行号** 平行数组 |

（原文 **§14.2** 多为从 jlox 过渡到 clox 的承上启下，与 §14.1 连贯阅读即可。）

---

## §14.1 什么是字节码（What is Bytecode?）

三种执行层次（对照 [ch2 编译之山](../../part01_welcome/chapter02_map-of-the-territory/02-map-of-the-territory.md)）：

| 方式 | 优点 | 缺点 |
|------|------|------|
| **AST 解释** | 实现直观、易调试 | 节点分散、缓存差、间接跳转多 |
| **原生机器码** | 最快 | 平台相关、编译复杂 |
| **Bytecode** | **紧凑连续**、缓存友好、**可移植** | 需 VM 或 JIT |

**Bytecode 定位**：介于源码 AST 与 CPU 机器码之间的 **中间表示（IR）**——类似 JVM / Python bytecode / Lua VM 指令。

```text
Source → [Compiler 前端] → Bytecode → VM interpret (或 JIT → 机器码)
```

**本仓库延伸**：[04_Learn-LLVM-17](../../../04_Learn-LLVM-17/) · LLVM IR 是更靠后的 IR 层。

---

## §14.3 指令块（Chunks of Instructions）

**`Chunk`** 核心结构（C）：

| 字段 | 作用 |
|------|------|
| **`code[]`** | 动态扩展的 **uint8_t** 数组，存 **Opcode + 操作数** |
| **`count` / `capacity`** | 已用长度 / 容量 |

**写入 API**（典型）：

- **`writeChunk(chunk, byte, line)`** — 追加一字节，同步记录行号。
- **`emitByte`** / **`emitReturn`** — 编译器侧封装。

**Opcode 枚举**（随章节增长）：

```c
typedef enum {
  OP_CONSTANT,
  OP_ADD,
  OP_SUBTRACT,
  OP_MULTIPLY,
  OP_DIVIDE,
  OP_NEGATE,
  OP_RETURN,
} OpCode;
```

早期 Chunk 只支撑 **算术计算器**；后续章节不断 **扩展 opcode 表**。

---

## §14.4 反汇编块（Disassembling Chunks）

VM 是「黑盒」→ 需要 **`disassembleChunk()`** 把二进制打印成可读指令。

```text
0000  1 OP_CONSTANT    0 '1.2'
0002     OP_RETURN
```

| 用途 | 说明 |
|------|------|
| 开发调试 | 对照源码与生成字节码 |
| 单测 | 断言编译输出 |
| 对照 jlox | 无 AST `toString()`，靠 disassembler |

**`disassembleInstruction(chunk, offset)`**：读 opcode → switch → 打印操作数（如常量索引）。

---

## §14.5 常量（Constants）

字面量 **不嵌入** 指令流（避免变长指令、对齐麻烦）→ **常量池 (Constant Pool)**。

| 结构 | 说明 |
|------|------|
| **`constants[]`** | 并行动态数组，存 **`Value`**（早期多为 **double**） |
| **`OP_CONSTANT`** | 后跟 **1 字节索引**（0～255），从池中取常量压栈 |

```text
code:     OP_CONSTANT  0
constants[0] = 1.2
```

| 设计 | 原因 |
|------|------|
| 索引单字节 | 与 clox 后续「单操作数字节码」风格一致；常量过多时再扩展 |
| 池与 code 分离 | 指令长度固定、解码简单 |

**对照**：JVM `ldc`、LLVM **constant pool** 思想相同。

---

## §14.6 行号信息（Line Information）

运行时错误需报 **源码行号** → 与 **`code[]` 平行的 `lines[]`**。

| 规则 | 说明 |
|------|------|
| **逐字节** | `lines[i]` = `code[i]` 对应源码行 |
| 与 `writeChunk(..., line)` 同步 | 每条 emit 传入当前 **Scanner 行** |
| 运行时 | ip 指向出错指令 → 查 **`lines[ip - chunk->code]`** |

**代价**：多一个 int 数组 · **收益**：用户友好的 stack trace（无 AST 节点行号字段时仍可用）。

---

## Chunk 内存布局（小结）

```text
Chunk
├── code[]      Opcode + operands（主指令流）
├── lines[]     与 code 等长 · 源码行号
└── constants[] 常量池 · OP_CONSTANT 索引
```

---

## 本章速记

```text
§14.1  Bytecode：紧凑、可移植、VM 执行
§14.3  Chunk 动态 code[] · OpCode 枚举
§14.4  disassembleChunk 调试
§14.5  常量池 + OP_CONSTANT 单字节索引
§14.6  lines[] 平行数组报行号
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **15** | [chapter15 · VM](../chapter15_a-virtual-machine/) | **`ip`** · **`run()`** · 值栈 · 算术 |
| **16** | [chapter16 · Scanning](../chapter16_scanning-on-demand/) | 按需 Token |
| **17** | Compiling Expressions | Token → **编译进 Chunk** |

---

## 自测

1. 为什么 `1.2` 不直接编码进 `OP_ADD` 之类的指令里？
2. `lines[]` 为何按**字节**而非按「指令」索引？
3. 反汇编器在 clox 开发流程里相当于 jlox 的什么工具？

---

## 阅读进度

- [x] §14.1、§14.3～§14.6 结构梳理（本章笔记）
- [ ] 本地 clox：跑 disassemble 示例 Chunk
- [ ] 本章 Challenges
