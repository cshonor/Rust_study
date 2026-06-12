# 1.1 Alignment（对齐）

> 所属：**Types in Memory** · [← 章索引](./README.md)

类型的主要作用之一：告诉编译器如何**合法、高效**地解释某段内存里的比特。对齐是第一步。

## 自然对齐

- 硬件常以固定宽度（如 x86_64 上 **8 字节 word**）为粒度访问内存。
- 类型的**起始地址**应满足其 **alignment**（通常为 2 的幂；与大小相关但不必处处相等）。
- **非对齐访问 (misaligned access)**：更慢；在部分架构上为 **UB**。

## 复合类型

- `struct` 等的对齐通常取各字段对齐要求的**最大值**。
- 编译器插入 **padding** 以满足每个字段的对齐。

## 实践

- FFI、协议解析、`#[repr(packed)]` 前必须显式考虑对齐。
- 与 [02 布局](./02-layout.md)、[llvm_insight ch05](../../llvm_insight/part02_src_to_machine/chapter05_ir_advanced_type/README.md) 对照读 IR 中的 `align`。
