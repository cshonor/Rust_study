# 第 5 章 · 中间表示（Intermediate Representation, IR）

> **Engineering a Compiler 3e** · [02 编译器工程](../../README.md) · [本书目录](../../本书目录.md) · Part II 基础结构

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**IR = 前端与后端的桥梁** — 独立于源语言与目标机，表达「正在被翻译的程序」。本章讲 **IR 为何需要、如何分类（图/线/混合）**、**AST/DAG/CFG**、**栈机/三地址/ILOC**、**SSA**、**寄存器 vs 内存模型** 与 **符号表** — 决定后续能做多深的优化。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 为何使用 IR | [01-why-ir.md](./01-why-ir.md) |
| §2 | IR 分类（图 / 线 / 混合） | [02-ir-taxonomy.md](./02-ir-taxonomy.md) |
| §3 | 图示 IR（AST · DAG · CFG） | [03-graphical-ir.md](./03-graphical-ir.md) |
| §4 | 线性 IR（栈机 · 三地址 · ILOC） | [04-linear-ir.md](./04-linear-ir.md) |
| §5 | SSA 静态单赋值 | [05-ssa-form.md](./05-ssa-form.md) |
| §6 | 名字映射与内存模型 | [06-names-and-memory-models.md](./06-names-and-memory-models.md) |
| §7 | 符号表 | [07-symbol-tables.md](./07-symbol-tables.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch5 | 对照 |
|----------|------|
| AST | [ch4 SDT](../chapter04_context/04-syntax-directed-translation.md) · [CI jlox ch5](../../../01_Crafting-Interpreters/part02_jlox/chapter05_representing-code/README.md) |
| 栈机 / 字节码 | [CI clox Chunk](../../../01_Crafting-Interpreters/part03_clox/chapter14_chunks-of-bytecode/README.md) |
| SSA / LLVM IR | [04 Learn LLVM 17](../../../04_Learn-LLVM-17/README.md) · ch8～10 优化 |
| 符号表 | [ch4](../chapter04_context/README.md) · jlox ch11 Resolver |

---

## 逻辑脉络

为何 IR → 形态分类 → 具体 IR 族 → SSA → 命名/存储假设 → 符号表。
