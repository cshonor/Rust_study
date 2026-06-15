# 第 24 章 · Calling and Closures（调用与函数）

> 在线：[calling-and-closures.html](https://craftinginterpreters.com/calling-and-closures.html) · 中文：[第 24 章 调用和闭包](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch23 Jumping](../chapter23_jumping-back-and-forth/notes/23-jumping-back-and-forth.md)

---

## 本章定位

此前 clox 仅在 **顶层脚本** 运行；本章起支持 **`fun`、调用、递归、return、原生函数**。函数体有 **独立 Chunk + CallFrame**；参数与局部变量共享 **同一块栈窗口**（零拷贝传递）。

| 对比 | jlox ch10 | clox ch24（本节） |
|------|-----------|-------------------|
| 函数值 | **`LoxFunction`** | **`ObjFunction`** + Chunk |
| 调用 | 新 Environment | **CallFrame 栈** |
| 参数 | define 在环境中 | **与 caller 压栈参数重叠** |
| return | **`Return` 异常** | **`OP_RETURN`** 弹帧 |
| native | **`LoxCallable`** | **`ObjNative`** |

| 小节 | 主题 |
|------|------|
| **§24.1～§24.2** | **`ObjFunction`** · Chunk · arity |
| **§24.3** | **`CallFrame`** 栈 |
| **§24.4～§24.5** | **`OP_CALL`** · 参数/局部 **栈重叠** |
| **§24.6** | **`return`** · **`OP_RETURN`** |
| **§24.7** | **`ObjNative`** · **`clock()`** |

**（书中同章后续：Upvalue / 闭包——见 §24.8+；你回复「25」时可继续梳理。）**

---

## §24.1～§24.2 函数对象（Function Objects）

**`ObjFunction`**（堆对象，ch19 `Obj` 继承）：

| 字段 | 说明 |
|------|------|
| **`arity`** | 形参个数（与 jlox/ch10 **255 上限** 一致） |
| **`name`** | **`ObjString*`** |
| **`chunk`** | 该函数 **专属字节码** |

**编译 `fun name(a, b) { ... }`**：

- 新建 **`Compiler`** 嵌套编译函数体 → 产出 **`function->chunk`**。
- **`OP_CLOSURE`**（闭包章节）或常量池 **`OBJ_VAL(function)`** + 全局/局部 define（随进度而定）。
- 顶层 **`fun`** → **`OP_DEFINE_GLOBAL`** 等。

**运行时**：函数是一等 **`Value`**（`OBJ_VAL`）。

---

## §24.3 调用帧（Call Frames）

递归 / 嵌套调用需要 **每调用一层一份状态**：

```c
typedef struct {
  ObjFunction* function;
  uint8_t* ip;           // 当前指令指针（指向 chunk->code 内）
  Value* slots;          // 本帧局部变量在 vm.stack 上的起始
} CallFrame;
```

| VM 字段 | 作用 |
|---------|------|
| **`frames[FRAMES_MAX]`** | CallFrame 数组 |
| **`frameCount`** | 当前深度 |

**`ip` 不再全局唯一**：每个 frame 有自己的 **`ip`**； **`vm.frameCount-1`** 为当前帧。

**`slots`**：指向 **该函数第一个局部槽** 在 **`vm.stack`** 中的位置（含参数）。

---

## §24.4～§24.5 函数调用（Function Calls）

**`OP_CALL argCount`**（单字节参数个数）：

1. callee = stack 上 **argCount 个参数之下** 的那个 Value。
2. 必须是 **`ObjFunction` 或 `ObjNative`**。
3. 校验 **arity**。

### 栈重叠优化（核心）

调用前 caller 已：

```text
... | callee | arg1 | arg2 | ...   ← stackTop
```

**新 CallFrame**：

```text
frame.slots = callee 在栈上的位置
// arg1, arg2 恰好是 slots[1], slots[2] …
// 即「参数槽」= 被调函数「局部变量窗口」底部
```

| 效果 | 说明 |
|------|------|
| **零拷贝** | 参数传递 **不 memcpy** |
| 局部 **`var`** | 继续往 stack 上 grow，索引在 frame 内 |

**新帧**：`frame.ip = function->chunk.code`；`frameCount++`。

**对照**：真实 ABI 的 **栈传参**；LLVM 调用约定同样靠栈/寄存器窗口。

---

## §24.6 返回语句（Return Statements）

**`return expr;` / `return;`** → compile 值（或 nil）→ **`OP_RETURN`**。

**VM 执行 `OP_RETURN`**：

```text
result = pop()                    // 返回值
frameCount--                      // 弹帧
if frameCount == 0: 解释结束，result 留栈或打印
else:
  丢弃 callee..locals 栈段
  push(result) 给 caller
  ip 恢复 caller frame
```

| 对比 jlox ch10 §10.5 | clox |
|----------------------|------|
| **`throw Return`** 跳出 Visitor | **显式弹 CallFrame** |
| 无栈帧概念 | **结构化帧栈** |

---

## §24.7 原生函数（Native Functions）

**`ObjNative`**：`Obj` + **C 函数指针** `NativeFn`。

```c
typedef Value (*NativeFunction)(int argCount, Value* args);
```

| 要点 | 说明 |
|------|------|
| **`OP_CALL`** | 与 **`ObjFunction`** 共用路径 |
| **`callValue`** | 判别类型 → **`callNative`** |
| **`clock()`** | 内置唯一 native · 返回秒级时间 |

**注册**：全局 **`defineNative("clock", fn)`**（实现细节随 REPL 引导）。

**对照 jlox ch10 §10.2**：同样为性能测试准备 **`clock()`**。

---

## Call 路径总览

```text
compile call:
  args... → callee → OP_CALL n

run OP_CALL:
  校验 arity · 建 CallFrame
  slots 指向 callee · args 已在 slots[1..n]
  新 ip 跑 function chunk

OP_RETURN:
  pop frame · push result · 恢复 caller ip
```

---

## 本章速记

```text
§24.1–2 ObjFunction · chunk · arity
§24.3   CallFrame[] · ip + slots 每帧独立
§24.4–5 OP_CALL · 参数槽与 locals 窗口重叠
§24.6   OP_RETURN 弹帧 · 返回值压回 caller
§24.7   ObjNative · clock()
后续    Upvalue / 闭包（同章 §24.8+）
```

---

## 读后下一步

| 章 / 节 | 内容 |
|---------|------|
| **ch24 续** | **Upvalue** · **`OP_CLOSE_UPVALUE`** · 真·闭包 |
| **25** | [chapter25 · Objects](../chapter25_objects/) | 实例字段 |
| **26** | Garbage Collection | **mark-sweep** |

---

## 自测

1. 为什么 `CallFrame.slots` 指向 callee 而不是 arg1？
2. 递归时 `frameCount` 与 Java 调用栈的对应关系？
3. Native 与 Function 在 `OP_CALL` 里分歧在哪一步？

---

## 阅读进度

- [x] §24.1～§24.7 结构梳理（本章笔记）
- [ ] 画三次递归 `fact(n)` 的 frames 与 stack
- [ ] 继续阅读同章 Upvalue / 闭包小节
- [ ] 本章 Challenges
