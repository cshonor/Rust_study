# 2.1 Juggling Raw Pointers（裸指针）

> 所属：**Great Power** · [← 章索引](./README.md)

## `*const T` / `*mut T`

- **无**生命周期与别名保证；可为 null。
- **有效性**由开发者证明。

## 典型用途

- 自引用 / 难静态表达的生命周期（与 [第 8 章 Pin](../Chapter-08-Asynchronous-Programming/06-pin-unpin.md) 衔接）
- 与 `Arc` 等运行时管理寿命的结构
- FFI 边界

## `NonNull<T>`

「永非 null」成立时优先使用；利于 **`Option<NonNull<T>>` niche 优化**。

→ [第 2 章 · DST](../Chapter-02-Types/04-dst-wide-pointers.md)
