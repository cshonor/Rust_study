# 2. Propagating Errors（传播错误）

> [← 章索引](./README.md)

## `?` 与 `From`

`?` 除提前返回外，会按需要触发 **`From`** 做错误类型转换。

- 为库级 `enum` 实现 `From<io::Error>` 等 → 内部 `io::Result` 经 `?` 自然抬升。

**`From` vs `Into`**：

- 实现侧：优先 **`From<T>`**（自动得 `Into`）。
- 约束侧：泛型有时写 **`Into<Foo>`** 对调用方更宽。

## `Try` trait

`?` 围绕 **`Try`** 概念；除 `Result` 外也与 `Option`、部分 `Poll` 语境相关（以当前 Rust 稳定性为准）。

## `try { ... }` 块

将 `?` 的提前返回**限制在块内**，避免函数中间 `?` 跳过末尾清理；使用前查当前 **edition / stability**。

ER → [Item 03 Option/Result 变换](../../01-ER/Chapter-01-Types/Item-03-option-result-transforms/README.md)
