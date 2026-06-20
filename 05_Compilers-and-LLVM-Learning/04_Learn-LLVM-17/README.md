# 04 · Learn LLVM 17 · LLVM IR 透视

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
