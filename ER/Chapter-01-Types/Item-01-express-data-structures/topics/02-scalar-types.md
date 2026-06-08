# Item 1 · 02 标量类型

← [Item 1 目录](../README.md)

标量 = **单个值**的类型。

| 类型 | 要点 |
|------|------|
| `bool` | `true` / `false`，1 字节 |
| `f32` / `f64` | IEEE 754 浮点，32 / 64 位 |
| `()` | **单元类型**，唯一值 `()`；表示「无有意义返回值」，语义类似 C 的 `void` |
| `char` | **Unicode 标量值**，固定 **4 字节**；**不能与整数静默互转** |

```rust
let c: char = '中'; // 4 字节
// let x: i32 = c;  // ❌ 不允许
```

## 相关

- 整数规则 → [01-fundamental-types.md](./01-fundamental-types.md)
- 背诵 → [cheat-sheet.md](./cheat-sheet.md)
