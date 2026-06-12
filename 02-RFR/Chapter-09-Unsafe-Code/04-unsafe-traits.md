# 2.3 Implementing Unsafe Traits（unsafe trait）

> 所属：**Great Power** · [← 章索引](./README.md)

若**仅通过安全代码**误用 trait 实现即可内存不安全 → **`unsafe trait`** + **`unsafe impl`**。

## 标准库示例

- **`Send` / `Sync`** — 错误 impl 可在**安全代码**中制造数据竞争。

## `Unpin`

`Unpin` 本身是**安全 trait**；危险来自 **`Pin` + `!Unpin`**（如 `Pin::new_unchecked`）。

→ [第 8 章 · Pin](../Chapter-08-Asynchronous-Programming/06-pin-unpin.md)

ER → [Item 16](../../01-ER/Chapter-03-Concepts/Item-16-avoid-unsafe/README.md) · Book → [16.4 Send/Sync](../../00-Book/16-fearless-concurrency/16.4-Send与Sync.md)
