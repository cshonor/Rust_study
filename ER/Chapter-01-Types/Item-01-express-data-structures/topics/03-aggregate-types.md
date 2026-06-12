# Item 1 · 03 聚合类型

← [Item 1 目录](../README.md)

**定义**：把多个值组合成一个整体的类型。

| 类型 | 背一句 |
|------|--------|
| **数组** `[T; N]` | **编译期定长**、**同构**；`arr[i]` 越界 → **panic** |
| **元组** `(T1, T2, …)` | 定长、**异构**；`.0`、`.1` 访问；可返回多个值 |
| **结构体** `struct` | **命名字段**，自定义复合数据 |
| **元组结构体** | 有类型名、无字段名；`.0`、`.1` 访问 |

```rust
struct Point { x: i32, y: i32 }
struct Color(u8, u8, u8); // 元组结构体

// HFT：一根 K 线用 struct 绑在一起，比零散标量好管
struct Bar {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    ts_ms: u64,
}
```

标量（`f64`、`u64` 等）描述**单个量**；聚合类型把多个量**绑成一个类型**，字段布局编译期确定（见 [01 固定大小](./01-fundamental-types.md)）。

> 数组越界在 debug 通常 panic；release 下仍可能 panic——答题说「越界 panic」足够，别当成 C 式静默读错内存。

## 相关

- 用 enum 表达状态 → [04-enum-adt.md](./04-enum-adt.md)
- 设计案例 → [06-design-patterns.md](./06-design-patterns.md)
