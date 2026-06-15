# 第 6 章 · 过程抽象（Procedure Abstraction）

> **Engineering a Compiler 3e** · [02 编译器工程](../../README.md) · [本书目录](../../本书目录.md) · Part II 基础结构

## 状态

- [x] 已读（笔记整理）

---

## 一句话

编译器不仅要翻译语句，还要构建**运行时环境**。本章讲 **过程/函数** 的**控制抽象**、**活动记录（栈帧）**、**嵌套作用域寻址**、**参数/返回值传递**、**调用约定（precall/prologue/epilogue/postreturn）** 以及 **堆与 GC** — 程序能稳定运行的幕后机制。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 控制抽象与名字空间 | [01-control-and-name-space.md](./01-control-and-name-space.md) |
| §2 | 活动记录（Activation Record） | [02-activation-records.md](./02-activation-records.md) |
| §3 | 建立可寻址性 | [03-addressability.md](./03-addressability.md) |
| §4 | 过程间传值 | [04-parameter-passing.md](./04-parameter-passing.md) |
| §5 | 标准链接约定 | [05-call-linkages.md](./05-call-linkages.md) |
| §6 | 内存管理与 GC | [06-memory-and-gc.md](./06-memory-and-gc.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch6 | 对照 |
|----------|------|
| CallFrame / 栈 | [CI clox ch24 Call Frames](../../../01_Crafting-Interpreters/part03_clox/chapter24_calling-and-closures/03-call-frames.md) |
| 闭包 / upvalue | [CI clox ch25](../../../01_Crafting-Interpreters/part03_clox/chapter25_objects/README.md) |
| GC | [CI clox ch26 GC](../../../01_Crafting-Interpreters/part03_clox/chapter26_garbage-collection/README.md) |
| 栈 / 堆 | RFR [第 1 章内存](../../../02-RFR/Chapter-01-Foundations/) · [03-2 OS layout](../../../02-RFR/Chapter-01-Foundations/03-2-os-memory-layout.md) |
| 符号表 / 作用域 | [ch5 §7 符号表](../chapter05_ir/07-symbol-tables.md) · [ch4](../chapter04_context/README.md) |

---

## 逻辑脉络

过程是什么 → 栈帧 AR → 怎么找变量 → 怎么传参/返回 → 四段调用序列 → 堆 + GC。
