# 《Learn LLVM 17》13 章：零基础精简解读 + 学习取舍

全书 4 大板块 13 章。**目标**：服务 **Rust 底层、并发、性能**；**不必**全书通读；**不必**写 C++ 编译器。

**与仓库文件夹对齐**：见 **[README.md](./README.md)**；精读/浏览/跳过与下表一致。

---

## 新手最简清单

### 必须精读

第 2、4、5、7、10 章

### 浏览即可

第 1、3、6、9 章

### 整段跳过

第 8、11、12、13 章

---

## 与当前仓库学习路线的贴合顺序

1. **[04/01-atomic/](../../04-Async-Concurrency-Network/01-atomic/)**：原子、内存序、锁  
2. **[04/02-async_tokio/](../../04-Async-Concurrency-Network/02-async_tokio/)**  
3. **[04/03-rust_network_programming/](../../04-Async-Concurrency-Network/03-rust_network_programming/)**  
4. **本目录按上表精读**：用前面代码**反查 IR 与优化**

---

## 姊妹仓库 · C++ 对照（可选）

同一维护者的 C++ 笔记线：[cpp-learning-notes](https://github.com/cshonor/cpp-learning-notes)。**不必**先读完 C++ 再开 LLVM；但下列章节能减轻读 IR 时的概念断层：

| C++（cpp-learning-notes） | 本仓库 Rust | 与 Learn LLVM / 05 的关系 |
|---------------------------|-------------|---------------------------|
| `07-Cpp-Object-Model` | RFR 第 2 章 · Nomicon 布局 | 读懂 struct 在 IR 里的字段、对齐、vtable |
| `08-Cpp-Concurrency` | `04/01-atomic` | 对照 `atomic.load` / fence / 锁在 `.ll` 里的形态 |
| `04-Effective-Modern-C++` | RFR · ER 部分条目 | 移动/值语义 ↔ Rust 所有权（非 LLVM 必修） |
| 可选 `11-Modern-C++-Performance-Engineering` | `04` 低延迟 + 本书 ch07 优化 | O0/O3、缓存行、无锁（与 `ir_samples/optimize_compare/` 互补） |

**05 主线仍用 Rust 代码**：`llvm_insight_lab` + [04-Async](../../04-Async-Concurrency-Network/README.md) demo → `ir_samples/`；C++ 仓作**背景阅读**，不要求写 C++ 编译器。

---

## 速记

**精读** 2 · 4 · 5 · 7 · 10 · **浏览** 1 · 3 · 6 · 9 · **跳过** 8 · 11–13  
**IR 主线**：改 `llvm_insight_lab` → `emit=llvm-ir` → 片段进 `ir_samples/`
