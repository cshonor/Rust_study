# 第 30 章 · Optimization（优化）

> 在线：[optimization.html](https://craftinginterpreters.com/optimization.html) · 中文：[第 30 章 优化](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch29 Superclasses](../chapter29_wrappers-and-api/29-superclasses.md)

---

## 本章定位

全书 **Lagniappe（额外赠礼）** · **最后一章含新代码**。用 **Benchmark + Profiler** 找热点，实施两项 **底层微优化** —— 不改 Lox 语义，只压榨 **VM 实现**。

| 工具 | 用途 |
|------|------|
| **`clock()` native** | 基准计时（ch10 jlox / ch24 clox 已备） |
| **Profiler** | 指令 / 函数 **热点占比** |

| 小节 | 主题 |
|------|------|
| **§30.1～§30.2** | 哈希表 **掩码** 替 **`%`** |
| **§30.3** | **NaN boxing** · Value **16B → 8B** |

---

## §30.1～§30.2 哈希表探测优化（Hash Table Probe Optimization）

**Profiler 发现**：**哈希表探测** 中 **`hash % capacity` 取模** 占 **极高 CPU 时间**（方法表 · globals · fields · intern…）。

**前提**：**`capacity` 始终为 2 的幂**（ch20 扩容 **`capacity *= 2`** 已保证）。

**位运算替换**：

```c
// 慢
uint32_t index = hash % capacity;

// 快
uint32_t index = hash & (capacity - 1);   // 掩码 Masking
```

| 原理 | 说明 |
|------|------|
| **2^n 取模** | 等价 **保留低 n 位** |
| **`& (cap-1)`** | 单条 **AND** · 比 **IDIV** 快一个数量级 |
| **缓存 / 流水线** | 热点在 **inner loop** 时每条探测都省 |

**书中效果**：方法/变量密集 benchmark **~2×** 加速（仅改这一处）。

**对照 [ch20](../chapter20_hash-tables/20-hash-tables.md)**：开放寻址 + 线性探测 · 优化 **index 计算** 即优化 **整条 lookup 链**。

**本仓库延伸**：[04_Learn-LLVM-17](../../../04_Learn-LLVM-17/) · **O0 vs O3** · 编译器也会做类似 strength reduction。

---

## §30.3 NaN 装箱（NaN Boxing）

**问题**：ch18 **`Value` struct**（tag + union + **对齐**）常占 **16 字节** / 栈槽 · 缓存行浪费。

**思路**：把 **所有 Value 塞进单个 `uint64_t` / `double` 位型** —— **NaN boxing**（LuaJIT、JavaScript 引擎常用）。

### IEEE 754 Double 位布局（概念）

```text
64 bit: [ sign | exponent | fraction/mantissa ... ]
```

- **Quiet NaN（qNaN）**：指数全 1 · 尾数非 0 → **大量 NaN  payload 位** 可 **存数据**。
- **合法 double 数** · **NaN 编码值** · **非数指针** 可 **共存于 64 位**。

### clox 编码策略（书中）

| 值种类 | 编码方式 |
|--------|----------|
| **number** | 普通 **IEEE double** 位型 |
| **`nil` / `true` / `false`** | 特定 **NaN 立即数** 模式 |
| **Obj 指针** | 利用 **48 位有效地址**（x64 常见）· 塞进 **NaN payload** |
| **Sign bit 标记** | 区分 **指针 boxed 值** vs **纯 double**（书中用符号位作 tag 手段之一） |

**结果**：

```text
Value: 8 字节（一个 uint64_t）
  栈更紧凑 · 常量池更小 · cache line 装更多 slot
  → 解释器整体再提速（书中给出显著 delta）
```

| 代价 | 说明 |
|------|------|
| **调试难度** | 位 hack · **`printValue` 需解码** |
| **可移植性** | 假设指针宽度 / NaN 语义 |
| **IS_NUMBER 等宏** | 改为 **位测试** 而非 **`type` 枚举** |

**对照 ch18 [tagged union](../chapter18_types-of-values/18-types-of-values.md)**：教学清晰 **→** 生产性能 **NaN tag**；同一语义两种表示。

---

## 优化方法论（全书收束）

```text
1. 正确实现 clox（ch14～29）
2. clock() 基准测试
3. Profiler 找热点（often hash %, dispatch, alloc）
4. 语义不变微优化（mask, NaN box, OP_INVOKE 属编译期）
5. 再测 · 确认 win
```

---

## 全书 30 章能力地图

```text
Part I   Lox 语言 + 编译之山
Part II  jlox: Scan → AST → Resolver → Tree-walk（ch4～13）
Part III clox:
  Chunk + VM + Scanner
  Pratt 单遍编译
  Value / String / Table / GC
  Locals · Control flow · Calls · Closures
  Classes · Methods · Inheritance · super
  ch30: mask + NaN boxing
```

| 你实现了 | 技术栈 |
|----------|--------|
| **jlox** | Java · AST · Visitor · Environment |
| **clox** | C · Bytecode · Stack VM · Mark-Sweep · OOP |

---

## 本章速记

```text
§30.1–2  cap=2^n → index = hash & (cap-1) · ~2× on hash-heavy
§30.3     NaN box · 64-bit Value · ptr/bool/nil in NaN space
方法      benchmark → profile → peephole（ch28 invoke 同理）
```

---

## 读后延伸

| 方向 | 链接 |
|------|------|
| **LLVM 优化** | [04_Learn-LLVM-17](../../../04_Learn-LLVM-17/) |
| **编译器 SSA** | [02 编译器工程](../../../02_Compiler-Principles/) |
| **附录** | [Appendix I Lox 语法](../../backmatter/appendix01_lox-grammar/) |
| **Wrappers** | [ch29 原书 REPL/API](https://craftinginterpreters.com/wrappers-and-api.html) |

---

## 自测

1. 为何 `& (capacity-1)` 要求 capacity 是 2 的幂？
2. NaN boxing 后 `IS_NUMBER` 如何判断一个 `uint64_t`？
3. 若不做 NaN box，栈 256 深时多浪费多少字节（相对 8B slot）？

---

## 阅读进度

- [x] §30.1～§30.3 结构梳理（本章笔记）
- [x] **Crafting Interpreters 正文 30 章笔记链路贯通**
- [ ] 跑书中 benchmark · 对比 mask / NaN 前后
- [ ] 本章 Challenges · 全书 Challenges 复盘
