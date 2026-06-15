# 第 22 章 · Local Variables（局部变量）

> 在线：[local-variables.html](https://craftinginterpreters.com/local-variables.html) · 中文：[第 22 章 局部变量](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch21 Global Variables](../chapter21_global-variables/notes/21-global-variables.md)

---

## 本章定位

ch21 全局变量走 **哈希表按名查找**；ch22 引入 **块作用域 + 局部变量**，变量直接住在 **Value Stack 槽位** 上，读写 **O(1) 按索引**。

| 对比 | 全局 ch21 | 局部 ch22 |
|------|-----------|-----------|
| 存储 | **`vm.globals` 表** | **栈槽 slot** |
| 指令 | **`OP_GET/SET_GLOBAL`** | **`OP_GET/SET_LOCAL`** |
| 编译期 | 名字 → 常量池 | **`Compiler.locals[]`** 记录 slot + depth |
| 查找 | 运行时 hash | **编译期定 index** |

| 小节 | 主题 |
|------|------|
| **§22.1** | **`Local`** · 栈偏移 · `locals` 数组 |
| **§22.2～§22.3** | **`beginScope` / `endScope`** · shadowing |
| **§22.4** | **`OP_GET/SET_LOCAL`** |
| **§22.5** | **`var a = a;`** 两阶段初始化 |

---

## §22.1 表示局部变量（Representing Local Variables）

**核心思想**：局部变量 = **栈上固定槽位**，与 ch15 **Value Stack** 合一（非单独数组）。

**`Compiler` 编译期模拟栈**：

```c
typedef struct {
  Token name;
  int depth;      // 声明时的 scopeDepth；-1 表示已就绪可用
  bool isCaptured; // 闭包用（ch24 后续）
} Local;
```

| 字段 | 含义 |
|------|------|
| **`locals[]`** | 当前函数/块内已知局部变量列表 |
| **`localCount`** | 已登记数量 |
| **`scopeDepth`** | 当前块嵌套深度 |
| **slot index** | 通常 = 在 `locals` 中的下标 = 相对栈帧底的偏移 |

**编译表达式/声明时**：维护「栈深度」与 **`locals`** 同步——每声明一个局部，等价于 **预留一个栈槽**。

**对照 jlox [ch8](../../part02_jlox/chapter08_statements-and-state/notes/08-statements-and-state.md)**：`Environment` 链 + 按名查找；clox **编译期绑定 slot**。

**对照 jlox [ch11](../../part02_jlox/chapter11_resolving-and-binding/notes/11-resolving-and-binding.md)**：`distance` 静态绑定；clox 局部变量 **直接 emit slot 号**。

---

## §22.2～§22.3 块作用域与声明局部变量

**块作用域 API**：

```c
beginScope()  → scopeDepth++
endScope()    → scopeDepth--；弹出 depth > scopeDepth 的 locals
```

**`var name = init;` 在块内**：

1. **`addLocal(name)`** — 登记到 `locals`，depth = 当前 `scopeDepth`。
2. 若有 initializer → compile（此时变量槽可能 **尚未可用**，见 §22.5）。
3. **`markInitialized()`** — depth 标记完成，允许读写。

**Shadowing（遮蔽）**：

- **`resolveLocal(compiler, name)`**：从 **`localCount-1` 向前** 扫描 `locals`。
- **先命中最内层** 同名 → 内层遮蔽外层。
- 未找到 →  fall through 到 **`resolveUpvalue`**（ch24）或 **全局**。

**`endScope`**：丢弃本层 locals（栈槽在运行时会随 **`OP_POP`/帧弹出** 回收，编译期只减 `localCount`）。

---

## §22.4 使用局部变量（Using Locals）

| 指令 | 操作 |
|------|------|
| **`OP_GET_LOCAL slot`** | `push(stack[slot])`（相对当前 CallFrame 底，ch24 前为单一顶层帧） |
| **`OP_SET_LOCAL slot`** | `stack[slot] = peek()` |

| 优势 | 说明 |
|------|------|
| **单字节 slot** | 无字符串 · 无 hash |
| **O(1)** | 直接索引 VM 栈 |
| **缓存友好** | 栈数组连续访问 |

**标识符编译**：

```text
resolveLocal → emit OP_GET_LOCAL index
canAssign + '=' → emit OP_SET_LOCAL index
未解析到局部 → OP_GET/SET_GLOBAL（ch21）
```

---

## §22.5 另一个作用域边缘情况（Another Scope Edge Case）

**问题**：`var a = a;`

- 右侧 `a` 应是 **外层** 的 `a`，不是 **正在声明、尚未初始化** 的内层槽。
- 若声明后立即允许读同一 slot → **读到未定义 / nil 错误**。

**两阶段**：

| 阶段 | 行为 |
|------|------|
| **`addLocal`** | 占用 slot，**depth 仍标记「未初始化」** |
| compile initializer | 允许 resolve **外层** `a`（内层槽不可见） |
| **`markInitialized`** | 初始化完成后才允许 **GET/SET 本 slot** |

**`resolveLocal`**：若 local 的 **`depth == -1`（未 init）** 且正在读变量 → **跳过**，继续向外找。

**对照**：TDZ（Temporal Dead Zone）的简化版语义；Rust 也有「声明 vs 初始化」规则。

---

## 局部 vs 全局（小结）

```text
全局:  名字 ──intern──► OP_*_GLOBAL ──hash──► vm.globals
局部:  名字 ──compile──► slot index ──O(1)──► stack[slot]
```

---

## 本章速记

```text
§22.1  locals[] · scopeDepth · slot = 栈槽
§22.2–3 begin/endScope · 从后向前 resolve · shadowing
§22.4   OP_GET/SET_LOCAL 单字节索引
§22.5   var a=a · 未初始化 vs markInitialized
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **23** | [chapter23 · Jumping](../chapter23_jumping-back-and-forth/) | **回填** · `OP_JUMP` · 控制流 |
| **24** | Calling and Closures | **CallFrame** · **`OP_CALL`** |
| **11** jlox | Resolver | distance ↔ clox slot/upvalue |

---

## 自测

1. 为什么局部变量不需要放进常量池里的名字？
2. `endScope` 时编译器丢弃 locals，运行时栈如何同步变短？
3. `var a = a` 若不做两阶段，内层 `a` 解析会错成什么？

---

## 阅读进度

- [x] §22.1～§22.5 结构梳理（本章笔记）
- [ ] 手工 trace 嵌套 `{}` 的 locals 与 scopeDepth
- [ ] 本章 Challenges
