#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

LLVM_README = """# 04 · Learn LLVM 17 · LLVM IR 透视

> 所属：[Compilers & LLVM Learning](../README.md)（仓库编号 **05**）  
> 与 RFR **第 2、第 10 章** 对照读 IR；**不学 LLVM C++ 源码**。  
> 前置实战：[04-Async-Concurrency-Network](../../04-Async-Concurrency-Network/README.md)

**笔记 + 可运行 crate `llvm_insight_lab` + `ir_samples/`** 分目录完成。

---

## 统一 IR 实验 crate

包名：**`llvm_insight_lab`**。

在**仓库根**执行：

```bash
cargo build --manifest-path 05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/Cargo.toml
cargo rustc --manifest-path 05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/Cargo.toml -- --emit=llvm-ir
```

生成物通常在 `target/debug/deps/llvm_insight_lab-*.ll`。片段复制到 `ir_samples/`（见 [ir_samples/README.md](./ir_samples/README.md)）。

### 建议对照实验

1. 改 `src/lib.rs` 中 `Ordering`，diff O0/O3 → `ir_samples/optimize_compare/`。
2. 从 [04/01-atomic](../../04-Async-Concurrency-Network/01-atomic/) 摘短逻辑 → `ir_samples/atomic_ir/`。
3. 从 [04/02-async_tokio](../../04-Async-Concurrency-Network/02-async_tokio/) 导出 → `ir_samples/async_tokio_ir/`。

---

## 速记

**精读** ch02 · ch04 · ch05 · ch07 · ch10 · **浏览** ch01/03/06/09 · **跳过** ch08 · part04  
**取舍详表** → [Learn-LLVM-17-学习取舍.md](./Learn-LLVM-17-学习取舍.md)
"""

LLVM_QUQU = """# 《Learn LLVM 17》13 章：零基础精简解读 + 学习取舍

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
"""

IR_SAMPLES = """# IR 样本归档（`ir_samples/`）

从 **`05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/src/lib.rs`** 或从 **[04-Async-Concurrency-Network](../../04-Async-Concurrency-Network/README.md)** 下三书代码导出 `.ll` 后，将**脱敏、可复现**片段放在此目录。

## 子目录

- `atomic_ir/` · `async_tokio_ir/` · `network_ir/` · `optimize_compare/`（O0/O3 成对）
"""

PART02 = """# Part 02 — 源码到机器码（核心主攻）

| 目录 | 章节 | 策略 |
|------|------|------|
| `chapter04_ir_basic` | 第 4 章 IR / SSA | **精读** |
| `chapter05_ir_advanced_type` | 第 5 章 复合类型 | **精读** |
| `chapter07_ir_optimize` | 第 7 章 优化 | **精读** |

从 [04-Async-Concurrency-Network/](../../04-Async-Concurrency-Network/README.md) 抄最小复现进 `src/lib.rs`，`.ll` 复制到 `ir_samples/`。
"""

for rel, text in [
    ("05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/README.md", LLVM_README),
    ("05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/Learn-LLVM-17-学习取舍.md", LLVM_QUQU),
    ("05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/ir_samples/README.md", IR_SAMPLES),
    ("05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/part02_src_to_machine/README.md", PART02),
]:
    p = ROOT / rel
    p.write_text(text, encoding="utf-8", newline="\n")
    print("wrote", rel)
