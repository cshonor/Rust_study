# 3.4 Casting（布局与转换）

> 所属：**Great Responsibility** · [← 章索引](./README.md)

## `#[repr(Rust)]`

**不保证**字段顺序与布局；`Foo<A>` 与 `Foo<B>` 不能假设可安全 **`transmute`**。

## 需要稳定布局

- **`repr(C)`** + 阅读参考手册
- FFI、协议解析 → [第 2 章 Layout](../Chapter-02-Types/02-layout.md)

## 指针 cast

- 对齐、provenance（Miri strict provenance）须满足
- `usize` ↔ 指针并非随意可逆

Book → [19.3 高级类型](../../00-Book/19-advanced-features/19.3-高级类型.md)
