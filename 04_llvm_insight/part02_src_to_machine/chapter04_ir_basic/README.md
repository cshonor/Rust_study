# Chapter 04 — LLVM IR 基础语法与 SSA

| 项目 | 说明 |
|------|------|
| **学习策略** | **精读（重点）** |
| **对应书** | 《Learn LLVM 17》第 4 章 |

## 本目录用途

- **吃透** IR 文本、`module` / `function` / **基本块** / **SSA**。  
- **实操**：以仓库根 `04_llvm_insight/src/lib.rs` 为主战场，改内存序、增函数后反复 `cargo rustc … --emit=llvm-ir`。
- **04 分发对照**：`lib.rs` 中 `process_static` / `process_dyn` → 笔记 [`notes/04_dispatch_static_vs_dyn.md`](./notes/04_dispatch_static_vs_dyn.md) · IR `ir_samples/optimize_compare/04_dispatch_O*.ll`

## 目录约定

| 子目录 | 说明 |
|--------|------|
| `notes/` | IR 语法摘录、**04_dispatch_static_vs_dyn**、与 LangRef 链接 |
| `code/` | 可选：书中跟写片段；主实验仍推荐用上级 `src/lib.rs` |

## IR 归档

将 `.ll`（可只截取相关 `define`）复制到 `ir_samples/optimize_compare/` 或 `atomic_ir/`，文件名建议：`04_<主题>_<opt-level>.ll`（已有 `04_dispatch_O0.ll` / `04_dispatch_O3.ll`）。
