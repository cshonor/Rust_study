# Item 1 · 05 Option 与 Result

← [Item 1 目录](../README.md)

## 内建枚举

| 类型 | 变体 | 语义 | 替代什么 |
|------|------|------|----------|
| **`Option<T>`** | `Some(T)` / `None` | 值可能存在或缺失 | **null**、空指针 |
| **`Result<T, E>`** | `Ok(T)` / `Err(E)` | 成功或失败 | **异常**（可恢复错误） |

## 四条实用原则

1. **缺失值用 `Option<T>`**——不用 `-1`、`nullptr` 等哨兵。
2. **可失败操作用 `Result<T, E>`**——不用魔法返回值或全局错误码。
3. **库 API 有诊断价值时保留 `Result`**——别偷换成 `Option`（Item 3 展开）。
4. **与 `?`、转换方法配合**——见 [Item 3](../../Item-03-option-result-transforms/README.md)、[Item 4](../../Item-04-idiomatic-error-types/README.md)。

## `Result` 与 `Error` trait（预告）

`Result<T, E>` 里的 **`E` 不强制**实现 `std::error::Error`，但自定义错误类型**惯例**应实现它，以便：

- 用 `{}`（`Display`）给用户看、用 `{:?}`（`Debug`）给开发者调试
- 通过 `source()` 关联底层原因（错误链）
- 与 `?`、`From`、日志库、`anyhow` 等生态对齐

| 谁实现了 `Error`？ | 说明 |
|--------------------|------|
| 标准库 | 如 `std::io::Error`、`ParseIntError` 等已默认实现 |
| 自定义类型 | 手写 `impl Error`，或用 **`thiserror`** 自动派生（见 [Item 4](../../Item-04-idiomatic-error-types/README.md)） |

高频交易里：行情查询缺数据 → `Option`；连接超时、下单被拒 → `Result<HftError>`，把精力放在**错误怎么处理**上，而不是手写 `Display` / `Error` 样板。

## 相关

- Book 深入 → [9.2 Result](../../../Book/09-error-handling/9.2-Result-与可恢复的错误.md)
- `Option` 嵌套陷阱 → [07-pitfalls.md](./07-pitfalls.md)
