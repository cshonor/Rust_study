# 第 2 章 · A Map of the Territory（领域地图） · §2.1 语言的组成部分（The Parts of a Language）

← [本章目录](./README.md) · 上一节：[00-overview.md](./00-overview.md) · 下一节：[02-shortcuts-and-alternate-routes.md](./02-shortcuts-and-alternate-routes.md)

---

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

→ **Rust/HFT 分层对照**（上山前端 + 下山后端 · `no_std` · `dyn` · FFI）：[04-rust-hft-编译流水线对照.md](./04-rust-hft-编译流水线对照.md)

---
