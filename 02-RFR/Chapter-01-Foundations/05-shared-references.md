# 3.1 Shared References（共享引用 `&T`）

> 所属：**Borrowing and Lifetimes** · [← 章索引](./README.md)

## 语义

- **只读**借用：在引用存活期内，通过其它**合法**路径不得突变该内存（除非走 [07 内部可变性](./07-interior-mutability.md)）。
- 可同时存在多个 `&T`，但均受**别名协议**约束。

## 与优化器（LLVM）

共享引用给优化器 **no-mutation** 假设：

- 同一 `*r` 的重复读可能被合并或缓存到寄存器。
- 与 RFR 第 1 章「借用不仅是语法」一致——也是给 [llvm_insight](../../llvm_insight/part02_src_to_machine/chapter04_ir_basic/README.md) 读 IR 时的线索。

## 常见误区

| 误区 | 纠正 |
|------|------|
| 「有 `&T` 就完全不能改」 | 内部可变性 / `UnsafeCell` 是显式例外 |
| 「`&T` 就是 C 的 `const T*`」 | Rust 还编码生命周期与别名，不是裸指针 |

Book → [4.2 引用与借用](../../00-Book/04-ownership/4.2-引用与借用.md)
