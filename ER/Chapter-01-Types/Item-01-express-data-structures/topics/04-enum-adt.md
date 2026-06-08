# Item 1 · 04 枚举与 ADT

← [Item 1 目录](../README.md)

## 定义

**枚举 = ADT（代数数据类型）**——一个类型只能是**若干互斥变体之一**，变体可携带**不同类型、不同数量**的数据。

**本质**：C 的 tag enum + **类型安全** tagged union。

## 三种变体

| 变体形式 | 示例 |
|----------|------|
| 单元变体 | `Quit` |
| 元组变体 | `Write(String)` |
| 结构体变体 | `Move { x: i32, y: i32 }` |

```rust
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}
```

## 与 `match`

- 必须**穷尽**所有变体，否则不编译。
- 公开 `enum` 加变体 = **破坏性变更** → 见 [07-pitfalls.md](./07-pitfalls.md)

## 相关

- 内建 enum → [05-option-result.md](./05-option-result.md)
- 重构案例 → [06-design-patterns.md](./06-design-patterns.md)
