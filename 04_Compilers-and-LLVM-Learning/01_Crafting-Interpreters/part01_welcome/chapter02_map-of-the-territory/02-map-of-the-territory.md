# 第 2 章 · A Map of the Territory（领域地图）

> 在线：[a-map-of-the-territory.html](https://craftinginterpreters.com/a-map-of-the-territory.html) · 中文：[第 2 章 领土地图](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part I · [part01_welcome](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch01 Welcome](../chapter01_introduction/01-welcome-introduction.md)

---

## 本章定位

第二章**仍不写实现代码**，而是给出**高层架构图**：从人类可读的**源代码**到**机器执行**，中间要经过哪些阶段。作者用 **「攀登山峰再下山」** 比喻整条流水线（与封面「编译之山」一致）。

| 小节（原文结构） | 主题 |
|------------------|------|
| **§2.1** The Parts of a Language | 流水线各站：Scanning → Parsing → IR → Code Gen → VM → Runtime |
| **§2.2** Shortcuts and Alternate Routes | 单遍编译、树遍历解释、转译——不必走完整「上山又下山」 |

读完本章 = 拥有**语言实现全局观**；Part II 起将逐一攻克山上的节点。

---

## §2.1 语言的组成部分（The Parts of a Language）

完整 **Pipeline**（那座「山」上的途径点）：

```text
Source Code（源码）
    ↑ 上山 · 前端
    Scanning / Lexing     → Tokens
    Parsing               → AST
    （语义分析 / 优化等）  → 常在此后进入 IR
    Intermediate Rep (IR) → 便于优化与后端
    ↓ 下山 · 后端
    Code Generation       → 机器码或字节码
    Virtual Machine       → 可选：跨平台执行字节码
    Runtime               → 类型检查、GC、内置服务等
    Machine Code / 运行结果
```

### §2.1.1 扫描 / 词法分析（Scanning / Lexing）

| 项目 | 说明 |
|------|------|
| **位置** | 流水线**第一步** |
| **输入** | 源代码**字符流** |
| **输出** | **Token**（词法单元） |
| **例子** | 数字 `123`、字符串 `"hi!"`、标识符 `min` |
| **丢弃** | 无意义空白、注释等 |

**本书对应**：Part II **ch4 Scanning**（jlox）· Part III **ch16 Scanning on Demand**（clox）。

**本仓库**：**03**《自制编译器》词法 · `00-Book` 宏/token 概念。

---

### §2.1.2 解析（Parsing）

| 项目 | 说明 |
|------|------|
| **作用** | 赋予代码**语法结构**（像句子成分分析） |
| **输入** | 扁平 **Token 序列** |
| **输出** | **AST**（抽象语法树），反映嵌套层级 |

**本书对应**：Part II **ch5～6**（表示代码 + 解析表达式）起建 jlox 的 AST。

---

### 中间表示（Intermediate Representations, IR）

- 将前端产物转为**更便于后续处理**的中间数据结构。
- 常见分工：
  - **前端 IR**：贴近源语言（AST 可视为一种 IR）。
  - **后端 IR**：贴近目标机器；便于**优化**与**代码生成**。

**本书对应**：

- jlox：以 **AST** 为主，不单独建工业级 IR。
- clox：**字节码 chunk** 即本书的 IR 形态。
- **04 Learn LLVM 17**：Rust → **LLVM IR**（SSA）· 对照 RFR 第 2 章分发 · ch07 优化。

**本仓库联想**：RFR [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md) · [03-2 OS/LLVM 内存布局](../../../../02-RFR/Chapter-01-Foundations/03-2-os-memory-layout.md) 里的 `alloca` / heap。

---

### 代码生成（Code Generation）

- 进入后端 **「下山」**：表示越来越**原始**。
- 产出 CPU（或 VM）能执行的**底层指令**。

**路径分支**：

| 目标 | 结果 |
|------|------|
| 直接生成**机器码** | 快，但绑定具体芯片 |
| 生成**字节码** + VM | 可移植（见 §2.1.7） |

**本书对应**：Part III clox 从 **ch17 Compiling Expressions** 起把 AST/语法编译为字节码。

---

### §2.1.7 虚拟机（Virtual Machine）

- 若只生成**某一芯片**的机器码 → 语言**丧失跨平台**。
- 常见解法：针对**假想芯片**设计**便携式指令** → **字节码（Bytecode）**。
- 用**软件模拟**的执行引擎 = **虚拟机**。

**本书对应**：Part III 全程 **`clox`** = 字节码 + VM（ch14～15 起）。

**本仓库联想**：JVM / WASM / Python VM；RFR 第 8 章 async 状态机与「解释执行循环」可类比 VM 主循环。

---

### §2.1.8 运行时（Runtime）

- 程序**真正跑起来**后的支撑层。
- 职责示例：
  - 动态**类型检查**
  - **垃圾回收（GC）** 等内存服务
  - 内置函数、模块加载等

**本书对应**：jlox 借 **Java GC**；clox **ch26 Garbage Collection** 自实现 GC。

**本仓库**：RFR 第 1 章内存区域 · 第 9～10 章 unsafe/并发 · **Nomicon** 布局与有效性。

---

## §2.2 捷径与替代方案（Shortcuts and Alternate Routes）

并非每种语言都要走完 §2.1 的**完整长线**；常见「抄近路」：

### §2.2.1 单遍编译器（Single-pass compilers）

| 项目 | 说明 |
|------|------|
| **做法** | 解析、语义分析、代码生成**交织**；解析到一个语法块**立刻**输出目标代码 |
| **特点** | **不在内存中保留完整 AST** → 极省内存 |
| **历史** | 早期 **C**、**Pascal** 等在资源受限环境下的设计 |

**对比本书**：CI 为教学**刻意走多遍、保留 AST/字节码**，便于理解每一站。

---

### §2.2.2 树遍历解释器（Tree-walk interpreters）

| 项目 | 说明 |
|------|------|
| **做法** | 解析成 **AST** 后**直接执行**；在树上逐节点、逐分支求值 |
| **特点** | 实现相对简单；通常**较慢** |
| **本书** | Part II **`jlox`** 即此路线（ch7 起求值遍历 AST） |

```text
Parse → AST → interpret(node) 递归/Visitor
```

**本仓库衔接**：理解 jlox 后再读 **03** 青木书「真编译器」全链路，对比「解释 vs 编译到二进制」。

---

### §2.2.3 转译器（Transpilers）

| 项目 | 说明 |
|------|------|
| **做法** | 不降到机器码/字节码；把源码**翻译**成另一种**高级语言** |
| **例子** | 某语言 → **C** / **JavaScript** 等，再交给现有工具链 |
| **特点** | 借目标语言生态；源码级可读性仍在 |

**对比**：**rustc** 不是 transpiler（降到 LLVM IR / 机器码）；**TypeScript → JS** 是典型的 transpile。

---

## 「编译之山」与本书两趟实现

| 路线 | 本书 | §2.2 对应 |
|------|------|-----------|
| **jlox** | Part II · AST 上树遍历 | **§2.2.2** |
| **clox** | Part III · 字节码 + VM + GC | **§2.1.7** + Runtime **§2.1.8** |
| **完整上山下山** | clox 更接近「编译到 IR 再执行」 | §2.1 全线（简化版） |

封面地图上的 **Optimizing / IR / Code Gen** → 本书 **ch30 Optimization** · 本仓库 **04** `ir_samples/optimize_compare/`。

---

## 本章速记

```text
§2.1  扫描→Token · 解析→AST · IR · 代码生成 · VM/字节码 · Runtime/GC
§2.2  单遍（省 AST）· 树遍历（jlox）· 转译（换高级语言）
全书  = 先 §2.2.2 再 §2.1.7 的 clox 路线
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **3** | [chapter03 · The Lox Language](../../chapter03_the-lox-language/) | **Lox 语言规格**——动手写 jlox 前必读 |
| **4** | Part II · Scanning | 流水线第一站：Token |

---

## 自测 / 对照（可选）

- [ ] 画一条从「Rust 源码」到「CPU 执行」的简化 pipeline（可含 `rustc` → LLVM IR → 机器码）。
- [ ] 说明 **jlox** 在 §2.1 的哪几站停下、**clox** 多走了哪几站。
- [ ] 各举 1 个 **transpiler** 与 **VM 语言** 的例子。
- [ ] 对照 RFR [03-2 OS/LLVM 内存布局](../../../../02-RFR/Chapter-01-Foundations/03-2-os-memory-layout.md)：`alloca`（栈）vs heap 分别更像 pipeline 哪一段的产物？

---

## 阅读进度

- [x] §2.1～§2.2 结构梳理（本章笔记）
- [ ] 本章 Challenges（若有）
- [ ] 进入 ch03 Lox 规格
