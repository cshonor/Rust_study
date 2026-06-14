# Compilers & LLVM Learning（编译器与 LLVM 学习）

> 与 Rust 主线 **`00-Book` → `02-RFR` → `01-ER` → `03-Rust_Nomicon`** 并列的**编译器专题**；按下面 **01 → 04** 顺序阅读笔记。

---

## 目录（四本书）

| # | 目录 | 书目 | 状态 |
|---|------|------|------|
| **1** | [01_Crafting-Interpreters](./01_Crafting-Interpreters/) | *Crafting Interpreters*（Bob Nystrom） | 占位 · 待整理 |
| **2** | [02_Compiler-Principles](./02_Compiler-Principles/) | 编译原理（龙书 / 虎书等） | 占位 · 待整理 |
| **3** | [03_Build-Your-Own-Compiler](./03_Build-Your-Own-Compiler/) | *Build Your Own Compiler* 类实战书 | 占位 · 待整理 |
| **4** | [04_Learn-LLVM-17](./04_Learn-LLVM-17/) | *Learn LLVM 17* · IR 实验 `llvm_insight_lab` | **已有** 笔记 + `ir_samples` |

---

## 与 Rust 笔记的衔接

| 编译器专题 | Rust 主线 |
|------------|-----------|
| 前端 / AST / 解释器 | `00-Book` 语法 · RFR 类型与分发 |
| 中间表示、优化 | **04_Learn-LLVM-17** ↔ RFR **第 2、9、10 章** · [`02-RFR/学习路径与章节对照.md`](../02-RFR/学习路径与章节对照.md) |
| unsafe / 内存布局 | `03-Rust_Nomicon` |

---

## 阅读顺序建议

```text
01 Crafting Interpreters（前端直觉）
  → 02 Compiler Principles（理论地图，可与 01 交错）
  → 03 Build Your Own Compiler（串联实战）
  → 04 Learn LLVM 17（IR + 与 Rust codegen 对照）
```

LLVM 可与 RFR **第 2 章**（布局、分发）**并行**精读。
