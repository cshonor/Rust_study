# 02 — Compiler Principles（优化与后端 · 鲸书）

> 所属：[Compilers & LLVM Learning](../README.md) · **以后深入 LLVM / 优化时再开**

| 项目 | 说明 |
|------|------|
| **俗称** | **鲸书**（Whale book） |
| **你口中的「编译器工程」** | 业界鲸书一般指 Muchnick 这本；**中文版书名**为《高级编译器设计与实现》（不是龙书） |
| **英文原名** | *Advanced Compiler Design and Implementation* |
| **作者** | Steven S. Muchnick |
| **中文版** | 赵克佳、沈志宇译 · 机械工业出版社 · [豆瓣](https://book.douban.com/subject/1400374/) |
| **定位** | 过程内 / 过程间优化、数据流、别名、寄存器分配、**商业编译器案例分析**（含 SPARC / POWER / x86 等） |
| **本目录** | 读书笔记占位；按章建 `notes/` |

## 什么时候读

- **不必现在买**：先完成 **01 在线** + **03 纸质** + **04 LLVM 实验**。
- **适合切入的时机**：
  - 读 `04_Learn-LLVM-17` ch07、`ir_samples/optimize_compare/` 时 **O0 vs O3** 看不懂；
  - 想写或读 **LLVM Pass**、理解 SSA 上的优化在「说什么」；
  - RFR **第 9、10 章** + Nomicon 之后，想把「编译器假设」与 IR 对上号。

## 与仓库其他部分

| 鲸书主题 | 本仓库对照 |
|----------|------------|
| 数据流 / 全局优化 | **04** `chapter07_ir_optimize` · `src/lib.rs` 改 `Ordering` |
| 中间表示 | **04** `chapter04`～`05` · RFR 第 2 章 |
| 存储层次优化 | RFR 第 2 章 layout · 第 10 章原子与内存序 |
| 商业编译器结构 | 对照 `rustc` / LLVM 文档 · RFR 第 13 章工具链 |

## 其他教材（未列入当前主线）

| 书 | 俗称 | 备注 |
|----|------|------|
| *Compilers: Principles, Techniques, and Tools* | 龙书 | 理论地图；需要时再补 |
| *Modern Compiler Implementation* | 虎书 | 与青木书路线相近，已用 **03** 覆盖实战 |
| *Engineering a Compiler* | 橡书 | Cooper/Torczon；工程向，非鲸书 |

## 待办

- [ ] 添加 `Learn-LLVM-17-学习取舍.md` 式「鲸书精读取舍」
- [ ] 章节目录 `本书目录.md`
