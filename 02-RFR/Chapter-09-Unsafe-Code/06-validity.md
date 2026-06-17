# 3.2 Validity（有效性）

> 所属：**Great Responsibility** · [← 章索引](./README.md)

← [05 什么会出错](./05-what-can-go-wrong.md) · 下一节 [07 Panic](./07-panics.md)

前置 → [05 三大契约](./05-what-can-go-wrong.md)

类型对「哪些位模式算合法值」有约束：

| 类型 | 约束直觉 |
|------|----------|
| **引用** | 非 null、对齐、指向合法对象、无悬垂 |
| **`bool`** | 仅 `0` 或 `1`；非法位型可破坏优化 |
| **enum** | 判别式 + 载荷须匹配变体 |

**Validity ≠ 已初始化** — Nomicon 区分更细；写 unsafe 须两者兼顾。

→ [04_Compilers-and-LLVM-Learning/04_Learn-LLVM-17](../../04_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/README.md) 读 IR 时可见 optimizer 假设
