# 第 1 章 · Welcome（Introduction）

> 在线：[introduction.html](https://craftinginterpreters.com/introduction.html) · 中文：[第 1 章 前言](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part I · [part01_welcome](../../README.md) · [本书目录](../../../本书目录.md)

---

## 本章定位

第一章 **Welcome** 是全书导览：在写任何解释器代码之前，完成**心理建设**与**宏观架构**——正式邀请读者踏上「探索编程语言内核」的旅程。

| 维度 | 本章在做什么 |
|------|----------------|
| **心态** | 拆掉「语言 = 魔法」的滤镜 |
| **地图** | 交代 Lox、jlox、clox 与三 Part 分工 |
| **方法** | 纯手工代码、不用 Lex/Yacc |
| **延伸** | Asides / Challenges / Design Notes |

---

## 1. 打破「魔法」滤镜

作者早年把写编译器、设计语言的人当成掌握深奥魔法的「巫师」。本章的核心态度是：

- 编程语言底层**没有魔法**，只是一行行的代码。
- 开发语言的人也是**普通人**；差别在于愿意把机制拆开、一行行写清楚。

**与 Rust 学习的关系**：读 RFR / Nomicon 时若觉得「编译器在施法」，可回到本章——后来你在 **04 LLVM** 里看到的 `alloca`、`invoke`、`call` 都是可读的工程选择，不是黑箱。

---

## 2. 全书结构：两次实现 Lox

本书从零开始、**两次**实现功能齐全的 **Lox** 语言：

| Part | 章次 | 实现 | 语言 | 侧重点 |
|------|------|------|------|--------|
| **I · Welcome** | 1～3 | — | — | 术语、编译之山、**Lox 规格** |
| **II · Tree-Walk** | 4～13 | **jlox** | Java | 概念清晰、语义正确；借 OOP + GC，**不追求速度** |
| **III · Bytecode VM** | 14～30 | **clox** | C | 内存管理、字节码编译；**贴近底层、更快** |

```text
jlox：先把「语言在说什么」写对（树遍历）
  ↓
clox：再把「机器怎么跑」写透（字节码 + VM + GC + ch30 优化）
```

**本仓库衔接**：

- Part II 前端直觉 → **03**《自制编译器》· `00-Book` 宏
- Part III VM / GC / Optimization → RFR 第 1～2 章内存 · **04** LLVM ch07

---

## 3. 阅读指南与内容特色

### 纯手工、真实代码

- 书中包含两个解释器所需的**每一行**真实代码。
- **不用** Lex、Yacc 等自动生成工具——避免「中间有一层工具生成，读者看不懂」的死角。
- 与本书封面「编译之山」一致：Scanning → Parsing → AST → … → Machine Code，全程可见。

### Asides（旁注）与 Challenges（挑战）

| 类型 | 作用 |
|------|------|
| **Asides** | 传记、历史背景，拓宽视野 |
| **Challenges** | 章末改代码的小练习，巩固当章机制 |

读某章时若时间紧：**正文 + 本章 Challenges 选做**即可；Asides 可后补。

### Design Notes（设计笔记）

多数书只讲**如何实现**语言，很少讲**如何设计**语言（特性是否易学、语法是否可读、命名是否自然等「人类学」问题）。

- 书中穿插 **Design Notes** 短文。
- **第一章末尾**示例：*What's in a Name?* —— 给编程语言**起好名字**的困难（不计入 30 章正文，但值得读）。

---

## 4. 本章要点速记

```text
没有魔法 → 两行解释器（jlox 清晰 / clox 底层）
不用生成器 → 每一行都亲手写
三 Part 分工 → 概念 → Java 树遍历 → C 字节码 VM
Design Notes → 实现之外还要想「人怎么用」
```

---

## 5. 读后下一步

| 顺序 | 章节 | 内容 |
|------|------|------|
| **2** | [A Map of the Territory](../../chapter02_map-of-the-territory/) | **编译之山**：扫描、解析、分析、IR、代码生成 |
| **3** | [The Lox Language](../../chapter03_the-lox-language/) | Lox 语法与语义（动手前必读规格） |
| **4+** | Part II | 从 Scanning 开始写 **jlox** |

---

## 6. 自测 / 挑战（可选）

- [ ] 用自己的话向他人解释：jlox 与 clox 各解决什么问题，为何要先 Java 后 C？
- [ ] 读 Design Note *What's in a Name?*，列举 2 个你熟悉的语言命名设计（好 / 坏各一）。
- [ ] 在 [`02-RFR/学习路径与章节对照.md`](../../../../02-RFR/学习路径与章节对照.md) 里标出：读 CI 时打算与 RFR 哪几章并行。
