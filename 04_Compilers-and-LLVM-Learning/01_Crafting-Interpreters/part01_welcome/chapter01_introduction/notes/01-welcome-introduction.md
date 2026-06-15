# 第 1 章 · Welcome（Introduction）

> 在线：[introduction.html](https://craftinginterpreters.com/introduction.html) · 中文：[第 1 章 前言](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part I · [part01_welcome](../../README.md) · [本书目录](../../../本书目录.md)

---

## 本章定位

第一章**不写代码**，只做三件事：**为什么要学**、**书怎么读**、**两趟实现（jlox → clox）预告**。读完应能建立信心，并知道 Part I～III 各干什么。

| 小节（原文结构） | 主题 |
|------------------|------|
| **§1.1** Why Learn This Stuff? | 动机 + 去魅 |
| **§1.2** How the Book is Organized | 排版、Code / Asides / Challenges / Design Notes |
| **§1.3** The First Interpreter | **jlox**（Java）预告；后文引出 **clox**（C） |

章末 **Design Note**：*What's in a Name?*（不计入 30 章正文）

---

## §1.1 为什么要学这些（Why Learn This Stuff?）

### 无处不在的小型语言（Little languages are everywhere）

- 世界不只有 C / Rust / Java 等**通用语言**。
- 大型项目里常有 **DSL**：配置格式、模板、查询语言、脚本层……
- 找不到现成库时，**自己写解析器**是极其实用的技能。

**本仓库联想**：`00-Book` 宏、`build.rs`、Cargo.toml、`#[derive]` 背后都是「小语言 + 处理器」；RFR 第 7 章宏也是同一谱系。

### 绝佳的编程锻炼（Languages are great exercise）

实现一门语言是对编程能力的**终极综合练习**，逼你真正会用：

| 数据结构 / 技巧 | 在解释器里典型出现 |
|-----------------|-------------------|
| 递归 | 解析、树遍历 |
| 动态数组 | Token 流、字节码 chunk |
| 树 | AST |
| 图 | 控制流、依赖（后文 clox 更多） |
| 哈希表 | 符号表、全局/局部绑定、字符串驻留 |

**与 RFR**：第 2 章 layout、第 1 章内存区域——读 clox 时会反复碰到。

### 还有一个原因（One more reason）

- 作者曾把写语言的人当成掌握魔法的「**巫师**」。
- 本章态度：**没有魔法**——底层只是一行行代码；做语言的人也是普通人。
- 目的：破除对编译器 / 解释器 / 「底层」的**胆怯**。

读 **04 LLVM** IR 或 **Nomicon** 时若发怵，可回到 §1.1 这句。

---

## §1.2 本书是如何组织的（How the Book is Organized）

### 三 Part +「一章一特性」

全书分 **三个部分**；每个**实现章**通常只聚焦**一个语言特性**：先讲概念，再写代码。

| Part | 章次 | 实现 | 语言 |
|------|------|------|------|
| **I · Welcome** | 1～3 | — | 术语 + Lox 规格 |
| **II · Tree-Walk** | 4～13 | **jlox** | Java |
| **III · Bytecode VM** | 14～30 | **clox** | C |

### 四个特色板块

| 板块 | 英文 | 作用 | 阅读建议 |
|------|------|------|----------|
| **代码** | The code | **每一行**真实实现代码，无遗漏 | 正文必读 |
| **旁注** | Asides | 历史、人物、趣闻 | 可跳过，不影响跟代码 |
| **挑战** | Challenges | 章末改/扩解释器 | 时间允许则做，吸收最扎实 |
| **设计笔记** | Design notes | **如何设计**语言（易学、可读、命名…） | 实现书少见；值得细读 |

**纯手工**：不用 Lex / Yacc 等生成器，避免「中间层看不见」的死角（与 §1.1 去魅一致）。

---

## §1.3 第一个解释器（The First Interpreter）

### jlox（Part II · Java）

- 第二个 Part 用 **Java** 写第一个解释器 **`jlox`**（树遍历）。
- 选 Java 的原因：
  - 足够**高级**，可专注**语言概念与语义**；
  - 写出**最简单、最干净**的实现；
  - 不被 C 层级的内存管理等细节拖住（GC、OOP 现成）。

```text
jlox 目标：把 Lox「说对了」—— 语义清晰优先，不追求速度
```

### clox（Part III · C）— 紧接 §1.3 后引出

- 第一个解释器**慢**；第三部分用 **C** 写 **`clox`**（字节码 VM）。
- 深入：**内存管理**、**字节码编译**、执行速度、GC 等底层机制。

```text
clox 目标：把 Lox「跑透了」—— 贴近机器、更快
```

**本仓库衔接**：jlox → **03**《自制编译器》前端直觉；clox → RFR 内存/类型 · **04** LLVM ch07 · ch30 Optimization。

---

## 本章速记

```text
§1.1  DSL 实用 + 数据结构综合练 + 没有魔法
§1.2  三 Part · 一章一特性 · Code/Asides/Challenges/Design notes
§1.3  jlox（Java 语义）→ clox（C 性能与底层）
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **2** | [chapter02 · A Map of the Territory](../../chapter02_map-of-the-territory/) | **编译之山**：扫描、解析、分析、IR、代码生成 |
| **3** | [chapter03 · The Lox Language](../../chapter03_the-lox-language/) | Lox 完整规格（写 jlox 前必读） |
| **4+** | Part II | 从 Scanning 动手 **jlox** |

---

## 自测 / Challenges（可选）

- [ ] 举一个你项目里的 **DSL**（配置、模板、宏展开……）。
- [ ] 用自己的话解释：为何先 **jlox** 再 **clox**？
- [ ] 读 Design Note *What's in a Name?*，各举 1 个「好命名 / 差命名」的语言特性。
- [ ] 在 [`02-RFR/学习路径与章节对照.md`](../../../../../02-RFR/学习路径与章节对照.md) 标出：读 CI 时与 RFR 哪几章并行。

---

## 阅读进度

- [x] §1.1～§1.3 结构梳理（本章笔记）
- [ ] Design Note *What's in a Name?*
- [ ] 本章 Challenges（若有）
