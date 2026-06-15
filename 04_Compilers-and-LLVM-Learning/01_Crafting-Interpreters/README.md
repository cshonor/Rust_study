# 01 — Crafting Interpreters

> 所属：[Compilers & LLVM Learning](../README.md) · **立刻能看、零成本**

![封面](./cover.png)

| 项目 | 说明 |
|------|------|
| **书** | *Crafting Interpreters*（Robert Nystrom / Bob Nystrom） |
| **英文免费在线** | [craftinginterpreters.com](https://craftinginterpreters.com/) |
| **中文在线（推荐）** | [craftinginterpreters-zh-jet.vercel.app](https://craftinginterpreters-zh-jet.vercel.app/)（[GuoYaxiang/craftinginterpreters_zh](https://github.com/GuoYaxiang/craftinginterpreters_zh)） |
| **本目录** | 读书笔记；按 Part I（树遍历 jlox）/ Part II（字节码 clox）建 `notes/` |

## 本书定位

Robert Nystrom 的 *Crafting Interpreters* 旨在**从零构建一门编程语言**。全书不靠 yacc/lex 等自动生成器，强调**手写代码**，把「语言实现」从黑箱里拉出来。

| 维度 | 内容 |
|------|------|
| **两个完整项目** | **jlox**（Java · Tree-walk 解释器）→ **clox**（C · 字节码虚拟机） |
| **学什么** | 扫描、解析、静态分析、中间表示、代码生成——封面「编译之山」上的各条路径 |
| **为什么值得读** | 显著提升对**数据结构**与**系统设计**的理解；语言无关，概念可迁移到 Rust |
| **本书产出** | 一份可照着走的**技术路线图**（前端直觉 → VM → 与 **03/04** 后端/IR 衔接） |

### 编译之山（封面地图）

```text
Source Code
    → Scanning → Tokens
    → Parsing → AST
    → Analysis / Optimizing → IR
    → Code Generation / VM → Bytecode / Machine Code
```

中间 plateau 上常见两条实现路线：**Tree-Walk Interpreter**（Part I）与 **Virtual Machine**（Part II）；Transpiling、Optimizing 等路径在 **02 编译器工程**、**04 LLVM** 中继续展开。

## 为什么先读这本

- **零成本、马上开**：中文在线完整可读，不必等纸质到货。
- **动手密度高**：两趟实现 Lox（Java 树遍历 + C 字节码 VM），建立「前端 → 运行时」直觉。
- **语言无关**：实现用 Java/C，概念直接迁移到 Rust / 本仓库后续 **03《自制编译器》**、**04 LLVM**。

## 与仓库其他部分

| CI 章节 | 仓库对照 |
|---------|----------|
| 词法 / 语法 / AST | **03** 自制编译器 · `00-Book` 宏（第 7 章） |
| 字节码 VM | RFR 第 8 章 async 状态机 · 第 2 章分发 |
| 闭包 / 类 | RFR 第 1、2、3 章 |

## 精读建议

| 范围 | 说明 |
|------|------|
| **Part I（jlox）** | 优先；建立 parser + 树遍历解释器全流程 |
| **Part II（clox）** | 字节码 + VM；与 **04** IR / 优化对照时再精读 |
| **Part III 语言设计** | 按需；与 **03** 设计子集语言时回看 |

## 待办

- [ ] 添加 `本书目录.md`（章 ↔ `notes/`）
- [ ] 确定 Part I only vs 全书精读
