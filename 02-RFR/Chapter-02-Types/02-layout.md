# 1.2 Layout（布局）

> 所属：**Types in Memory** · [← 章索引](./README.md)

布局描述：字段顺序、padding、`size` / `align` 如何在内存中排列。

## `repr` 一览

| `repr` | 含义（直觉） |
|--------|----------------|
| **`repr(Rust)`（默认）** | 编译器可重排字段以优化大小/访问；**不保证**跨版本布局稳定。 |
| **`repr(C)`** | 与 C 兼容：字段顺序固定，按 C 规则 padding；FFI / 稳定 ABI。 |
| **`repr(packed)`** | 尽量去掉字段间 padding；体积小，但易产生未对齐访问；协议解析等场景需谨慎。 |

## 工具

- `std::mem::size_of` / `align_of` / `offset_of`（nightly 或 crate）
- `std::alloc::Layout` — 与堆分配、自定义分配器相关

## 延伸

- 枚举 **niche optimization**、空指针优化 → [03 复合类型](./03-complex-types.md)
- 自定义布局与 unsafe → RFR 第 9 章

Book → [19.3 高级类型 · 内存布局](../../00-Book/19-advanced-features/19.3-高级类型.md)
