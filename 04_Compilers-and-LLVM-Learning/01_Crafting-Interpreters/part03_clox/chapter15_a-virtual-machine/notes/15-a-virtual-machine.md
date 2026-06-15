# 第 15 章 · A Virtual Machine（虚拟机）

> 在线：[a-virtual-machine.html](https://craftinginterpreters.com/a-virtual-machine.html) · 中文：[第 15 章 虚拟机](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch14 Chunks](../chapter14_chunks-of-bytecode/notes/14-chunks-of-bytecode.md)

---

## 本章定位

有了 **Chunk**（ch14），本章造 **执行引擎**：**栈式 VM** + **指令指针 ip** + **`run()` 主循环**。先实现 **算术计算器**，验证 bytecode 可跑。

| 对比 | jlox Interpreter | clox VM |
|------|------------------|---------|
| 驱动 | Visitor 递归 AST | **`while` + switch(opcode)** |
| 操作数暂存 | Java 调用栈 / 局部变量 | **显式 Value Stack** |
| 程序计数 | 隐式在递归深度里 | **`ip`** 显式指向当前指令 |

| 小节 | 主题 |
|------|------|
| **§15.1** | **`VM`** · **`ip`** · **`run()`** · switch dispatch |
| **§15.2** | **值栈 (Value Stack)** |
| **§15.3** | 算术 · **`OP_NEGATE`** · **`OP_ADD`** 等 |

---

## §15.1 指令执行机（An Instruction Execution Machine）

**`VM` 结构体**（核心字段）：

| 字段 | 作用 |
|------|------|
| **`chunk`** | 当前执行的 **`Chunk*`** |
| **`ip`** | **Instruction Pointer**，指向 **`chunk->code` 中下一条指令** |
| （§15.2）**`stack`** | 操作数 / 结果栈 |

**`interpret(chunk)`**：

```text
vm.chunk = chunk
vm.ip    = chunk->code   // 起始
return run()
```

**`run()` 骨架**：

```c
for (;;) {
  uint8_t instruction = *vm.ip++;
  switch (instruction) {
    case OP_CONSTANT: ... break;
    case OP_ADD:      ... break;
    ...
    case OP_RETURN:   return INTERPRET_OK;
  }
}
```

| 概念 | 说明 |
|------|------|
| **Fetch** | 读 `*ip++` 得 opcode |
| **Decode / Execute** | `switch` 分支（后续可换 **computed goto** 优化，ch30） |
| **循环** | 直到 **`OP_RETURN`** 或错误 |

**对照**：CPU 取指 → 译码 → 执行；VM 用 C **`switch`** 模拟 **dispatch table**。

---

## §15.2 值栈操作（A Value Stack Manipulator）

表达式求值需要 **暂存中间结果** → **后进先出** 的 **固定大小栈**（数组 + **`stackTop`** 指针）。

| 操作 | 说明 |
|------|------|
| **`push(value)`** | `*stackTop++ = value` |
| **`pop()`** | `return *--stackTop` |
| **`peek(offset)`** | 看栈顶下第 n 个（少弹栈） |

**为何栈式？**

- 二元运算自然顺序：**左操作数、右操作数入栈 → 运算符指令弹出并压回结果**。
- 与 **JVM、Forth、RPN 计算器** 同族。
- 局部「虚拟寄存器」= 栈槽，无需为每个临时量分配命名 slot（早期章节）。

**栈溢出**：固定 **`STACK_MAX`**；压栈前检查（运行时错误）。

**jlox 对照**：Visitor 返回值在 **Java 栈**；clox 把栈 **显式化**，便于后续 **帧指针 / call frame**（ch24）。

---

## §15.3 算术计算器（An Arithmetic Calculator）

实现 opcode（与 ch14 枚举对应）：

| 指令 | 行为 |
|------|------|
| **`OP_CONSTANT idx`** | 读 1 字节索引 → 从常量池取 **Value** → **push** |
| **`OP_NEGATE`** | pop 一个 → 取负 → push |
| **`OP_ADD` / `SUBTRACT` / `MULTIPLY` / `DIVIDE`** | pop 右、pop 左 → 运算 → push |
| **`OP_RETURN`** | 结束；栈顶为结果（调试时可 print） |

**执行示例** `-((1.2 + 3.4) / 5.6)`（概念栈变化）：

```text
CONSTANT 1.2    stack: 1.2
CONSTANT 3.4    stack: 1.2 3.4
ADD             stack: 4.6
CONSTANT 5.6    stack: 4.6 5.6
DIVIDE          stack: 0.821...
NEGATE          stack: -0.821...
RETURN
```

| 优雅之处 | 说明 |
|----------|------|
| 编译器 | 按表达式结构 **emit 序列**，无需寄存器分配 |
| VM | 每条指令 **O(1)** 栈操作 |

**除零等**：运行时检查 → 报错 + **行号**（ch14 **`lines[]`**）。

**仍缺**：变量、控制流、函数、对象——**ch17+** 逐步加 opcode。

---

## VM 与 Chunk 协作

```text
         ┌─────────────┐
         │   Chunk     │
         │ code[]      │◄── ip 逐字节前进
         │ constants[] │
         │ lines[]     │──► 报错行号
         └─────────────┘
                ▲
                │ fetch/decode
         ┌──────┴──────┐
         │     VM      │
         │  stack[]    │  中间结果 LIFO
         └─────────────┘
```

---

## 本章速记

```text
§15.1  VM · ip · run() · switch 分发
§15.2  Value Stack · push/pop · STACK_MAX
§15.3  OP_CONSTANT/NEGATE/ADD/.../RETURN · 栈式算术
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **16** | [chapter16 · Scanning](../chapter16_scanning-on-demand/) | **按需** Token · 关键字 DFA |
| **17** | Compiling Expressions | 源码 → **emit Chunk** |
| **18** | Types of Values | **NaN tagging** 统一 Value |

---

## 自测

1. `ip` 在读取 `OP_CONSTANT` 的操作数（索引字节）时如何推进？
2. 栈式 VM 执行 `a + b * c` 时，编译器应保证什么顺序？
3. clox 的 `run()` 与 jlox 的 `visitBinaryExpr` 在控制流上有何本质不同？

---

## 阅读进度

- [x] §15.1～§15.3 结构梳理（本章笔记）
- [ ] 手写一段 bytecode 用 VM 跑通四则运算
- [ ] 本章 Challenges
