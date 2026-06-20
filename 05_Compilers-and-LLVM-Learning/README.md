# Compilers & LLVM Learning（编译器与 LLVM 学习）

> 仓库编号 **05** · 前置专题：[04-Async-Concurrency-Network](../04-Async-Concurrency-Network/README.md)（atomic / async / network 实战后再做 IR 对照）  
> Rust 主线：**`00-Book` → `02-RFR` → `01-ER` → `03-Rust_Nomicon` → `04-Async` → 本目录**

---

## 我的选用（已确认封面）

| 阶段 | 做什么 | 书目 / 入口 |
|------|--------|-------------|
| **立刻、零成本** | 前端直觉：词法 / 语法 / 解释器 | [**Crafting Interpreters**](./01_Crafting-Interpreters/README.md) · 中文在线 |
| **纸质入门** | 从零做真编译器 + 链接 / 加载全链路 | [**《自制编译器》**](./03_Build-Your-Own-Compiler/README.md)（青木峰郎） |
| **对照 Rust** | IR、优化、与 codegen 对齐 | [**04_Learn-LLVM-17**](./04_Learn-LLVM-17/README.md) · 可与 RFR **第 2、9、10 章** 并行 |
| **以后深入** | SSA、优化、代码生成、读 LLVM Pass | [**《编译器工程》**](./02_Compiler-Principles/README.md) · Cooper/Torczon **第三版**（橡书） |

```text
04 实战：atomic → async_tokio → rust_network
  → 05 现在：01 Crafting Interpreters 中文在线（免费）
  → 买一本：03 《自制编译器》
  → 并行：04 Learn LLVM 17 + RFR 第 2 章（用 04 里写过的代码反查 IR）
以后：02 Engineering a Compiler 3e（编译器工程 / 橡书）+ LLVM Pass / O0 vs O3
```

> **命名**：口头「编译器工程」= Cooper *Engineering a Compiler*（**橡书**），不是 Muchnick **鲸书**《高级编译器设计与实现》。

---

## 目录（四本书）

| # | 目录 | 当前选用 | 状态 |
|---|------|----------|------|
| **1** | [01_Crafting-Interpreters](./01_Crafting-Interpreters/) | *Crafting Interpreters* · **中文在线** | 路线已定 · 笔记待整理 |
| **2** | [02_Compiler-Principles](./02_Compiler-Principles/) | **《编译器工程》** Cooper/Torczon **3e** | **以后** · 占位 |
| **3** | [03_Build-Your-Own-Compiler](./03_Build-Your-Own-Compiler/) | **《自制编译器》**（青木峰郎） | 目录已建 · 逐章笔记待读 |
| **4** | [04_Learn-LLVM-17](./04_Learn-LLVM-17/) | *Learn LLVM 17* · `llvm_insight_lab` | **已有** 笔记 + `ir_samples` |

---

## 与 Rust 笔记的衔接

| 编译器专题 | Rust 主线 |
|------------|-----------|
| 前端 / AST / 解释器 | `00-Book` 语法 · RFR 类型与分发 |
| 中间表示、优化 | **04_Learn-LLVM-17** ↔ RFR **第 2、9、10 章** · [`02-RFR/学习路径与章节对照.md`](../02-RFR/学习路径与章节对照.md) |
| IR 对照素材 | [`04-Async-Concurrency-Network/`](../04-Async-Concurrency-Network/README.md) 三书 demo → `ir_samples/` |
| unsafe / 内存布局 | `03-Rust_Nomicon` |

LLVM 可与 RFR **第 2 章**（布局、分发）**并行**精读；**原子 / async IR** 建议在 **04 专题** 有代码后再做 diff。

---

## 姊妹仓库 · C++ 对照（可选）

[C++ 学习笔记 cpp-learning-notes](https://github.com/cshonor/cpp-learning-notes) 与本仓库**同一套「按书分目录 + 笔记 + demo」**组织方式，编号 `01`～`09` 对应 Primer → Effective 系列 → 对象模型 → 并发 → C++20。

| 你关心 | C++ 仓 | 本仓库 Rust | 进入 05 / LLVM 前建议 |
|--------|--------|-------------|------------------------|
| 对象布局、vtable | `07-Cpp-Object-Model` | RFR 第 2 章 · Nomicon | **强烈建议**至少其一，再读 IR 第 4～5 章 |
| 线程、内存模型 | `08-Cpp-Concurrency` | [`04/01-atomic`](../04-Async-Concurrency-Network/01-atomic/) | 做完 04 再 diff `ir_samples/atomic_ir/` |
| 现代 C++ 语义 | `04-Effective-Modern-C++` | RFR · ER | 非 LLVM 必修；有助于理解优化与移动 |
| 低延迟 / 性能 | 可选 `11-Modern-C++-Performance-Engineering` | 04 三书 + 本书 ch07 | 与 O0/O3、`optimize_compare/` 对照 |

**05 不依赖 C++ 编译器**：IR 实验一律用 Rust（`llvm_insight_lab` + 04 demo）。C++ 仓仅作**平行背景**，详见 [04_Learn-LLVM-17 学习取舍](./04_Learn-LLVM-17/Learn-LLVM-17-学习取舍.md)。
