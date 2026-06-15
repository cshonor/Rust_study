# 第 23 章 · Jumping Back and Forth（来回跳转）

> 在线：[jumping-back-and-forth.html](https://craftinginterpreters.com/jumping-back-and-forth.html) · 中文：[第 23 章 来回跳转](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch22 Local Variables](../chapter22_local-variables/22-local-variables.md)

---

## 本章定位

字节码是 **一维扁平指令流** → **控制流 = 改 `ip`（跳转）**。本章实现 **`if` / `and`·`or` / `while` / `for`**，并引入经典 **回填 (Backpatching)**。

| 对比 | jlox ch9 | clox ch23 |
|------|----------|-----------|
| `if` | Visitor 递归分支 | **`OP_JUMP_IF_FALSE` + `OP_JUMP`** |
| `while` | Java `while` | **`OP_LOOP` 负偏移** |
| `for` | AST 脱糖 | **编译期脱糖为 while 结构** |
| `and`/`or` | 短路不求值右操作数 | **跳转 + 栈顶保留左值** |

| 小节 | 主题 |
|------|------|
| **§23.1** | **`if`** · 回填 |
| **§23.2** | **`and` / `or`** · 短路 |
| **§23.3** | **`while`** · **`OP_LOOP`** |
| **§23.4** | **`for`** · 脱糖 |

---

## §23.1 If 语句（If Statements）

**语义**：condition falsy → 跳过 then；可选 else。

**指令**：

| Opcode | 行为 |
|--------|------|
| **`OP_JUMP_IF_FALSE offset`** | pop 条件；若 **falsy** → **`ip += offset`** |
| **`OP_JUMP offset`** | 无条件 **`ip += offset`** |

**编译顺序（有 else）示意**：

```text
compile condition
JUMP_IF_FALSE → else分支
compile then
JUMP → 结束
else:
compile else
结束:
```

### 回填（Backpatching）

emit **`JUMP_*`** 时 **还不知道** 要跳过多少字节 → 先写 **占位 offset（如 0xFF）**，记录 **patch 地址**；分支编译完后再 **写回真实偏移**。

| API | 作用 |
|-----|------|
| **`emitJump(op)`** | 发 opcode + 占位 2 字节 offset，返回 patch 点 |
| **`patchJump(offset)`** | `当前ip - offset - 2` 写入真实跳转距离 |

**offset 单位**：通常为 **字节数**（跳过多少 code 字节），非「指令条数」。

**对照 jlox [ch9 §9.2](../../part02_jlox/chapter09_control-flow/09-control-flow.md)**：悬挂 else 靠 Parser；clox 靠 **跳转布局 + 回填**。

---

## §23.2 逻辑运算符（Logical Operators）

Lox：**短路** + **返回决定结果的原始值**（不一定是 bool）。

**`and`**（左 falsy → 整式 = 左）：

```text
compile left
JUMP_IF_FALSE → end        // 左 falsy：栈顶已是结果
POP                        // 左 truthy：丢弃，继续
compile right
end:
```

**`or`**（左 truthy → 整式 = 左）：

```text
compile left
JUMP_IF_FALSE → evalRight  // 左 falsy 才算右
JUMP → end                 // 左 truthy：栈顶保留
evalRight:
POP
compile right
end:
```

| 要点 | 说明 |
|------|------|
| 复用 **`OP_JUMP_IF_FALSE`** | 少 opcode |
| 栈顶 | 短路时 **左操作数留在栈上** 作为结果 |

**对照 jlox ch9 §9.3**：语义相同；实现从 Visitor 改为 **控制流 bytecode**。

---

## §23.3 While 循环（While Statements）

```lox
while (cond) body;
```

```text
loopStart:
  compile cond
  JUMP_IF_FALSE → exit
  compile body
  OP_LOOP → loopStart   // 往回跳
exit:
```

**`OP_LOOP` offset**：

- **负向/backward**：从当前 ip **回跳** offset 字节到 **loop 头**。
- 与 **`OP_JUMP`（向前）** 配对：条件失败 **向前跳出**； body 结束 **向后循环**。

**VM**：`ip -= offset`（实现细节以书中为准）。

---

## §23.4 For 循环（For Statements）

C 风格 **`for (init; cond; increment) body`** → **前端脱糖**，**无专属 `OP_FOR`**。

典型 lowering（与 jlox ch9 §9.5 同构）：

```text
init
loopStart:
  cond → JUMP_IF_FALSE exit
  body
  increment
  OP_LOOP loopStart
exit:
```

| 优点 | 说明 |
|------|------|
| VM 简单 | 只需 jump 族 + loop |
| 与 jlox 一致 | 语法糖不进入 IR |

**init 作用域**：若 init 是 **`var`**，整个 for 外层包 **`beginScope/endScope`**（变量作用域限于 for）。

---

## 跳转指令族（小结）

```text
OP_JUMP_IF_FALSE  条件假 → 向前跳过
OP_JUMP           无条件向前
OP_LOOP           无条件向后（循环）
        ↑ 偏移量在编译完成后 backpatch
```

---

## 本章速记

```text
§23.1  if · JUMP_IF_FALSE/JUMP · backpatch 占位再修补
§23.2  and/or 短路 · 栈顶留左值 · 复用条件跳转
§23.3  while · OP_LOOP 回跳
§23.4  for 脱糖为 init+while+increment
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **24** | [chapter24 · Calls](../chapter24_calling-and-closures/) | **CallFrame** · **`OP_CALL`** · return |
| **25** | Objects | 堆对象扩展 |

---

## 自测

1. 为什么回填偏移用「字节」而不是「指令序号」？
2. `a or b` 在左 truthy 时栈上最终剩几个值？
3. `for` 脱糖后，`continue`（若实现）应改哪几个 jump 目标？

---

## 阅读进度

- [x] §23.1～§23.4 结构梳理（本章笔记）
- [ ] 画一条 `if/else` 的 bytecode 布局与 patch 点
- [ ] 本章 Challenges
