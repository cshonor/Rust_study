# Item 4 · 核心知识点

← [Item 4 目录](../README.md)

### `std::error::Error` trait

- `Result<T, E>` 的 **`E` 不强制**实现 `Error`，但实现它是与生态对齐的**惯例**。
- 实现 `Error` 前必须已有：
  - **`Display`**（`{}`，给用户看）
  - **`Debug`**（`{:?}`，给开发者调试）

### `source()` 方法

- 默认返回 `None`；可重写以暴露**底层原因**：
  - `fn source(&self) -> Option<&(dyn Error + 'static)>`

### 孤儿规则（Orphan Rule）

- 只有「trait 或类型至少一方定义在当前 crate」才能 `impl`。
- **不能**写 `impl Error for String`（`String` 与 `Error` 都在标准库）。

---
