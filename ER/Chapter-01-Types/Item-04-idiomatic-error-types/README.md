# Item 4: Prefer idiomatic Error types

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：倾向使用符合惯例的 Error 类型  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-04-error-types](./demo/) · [item-04-core-error](./demo-core-error/)（no_std `Error`）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `Result`、`?` | [9.2 Result](../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |
| `Display` / `Debug` | [10.2 trait](../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| Newtype | [Item 6](../Item-06-newtype-pattern/README.md)（ER） |
| Option/Result 链式 | [Item 3](../Item-03-option-result-transforms/README.md)（ER） |

---

## 1. 核心知识点与关键定义

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

## 2. 逻辑脉络与知识点关联

```text
String 当错误（不行：无法 impl Error）
  → Newtype 包装（MyError(String)）
  → Enum 聚合子错误 + 自定义 source()
  → 库：具体 Enum + thiserror
  → 应用：Box<dyn Error> + anyhow
```

### Newtype 突破孤儿规则

- `String` 不能直接挂 `Error`。
- `struct MyError(String);` 是**新类型**，可在本 crate `impl Display + Error`。

### 枚举保留诊断细节

- 不同失败原因（I/O vs UTF-8 vs 业务）→ **`enum MyError { Io(...), Utf8(...), ... }`**
- 在 `source()` 里把内部 `io::Error` 等**链出去**。

### 特征对象 vs 一致性（Coherence）

- 想统一异构错误：`Box<dyn Error>` 包装。
- 若再手写 `impl<E: Error> From<E> for MyWrapper` 等，易与标准库 **blanket impl**（如 `From<T> for T`）**冲突** → 应用层优先用 **`anyhow`**。

---

## 3. 重点结论与实用要点

| 场景 | 建议 |
|------|------|
| **任何自定义错误** | 实现 `Error`（+ `Display` / `Debug`），别裸 `String` |
| **配合 `?`** | 为子错误实现 `From`（如 `From<io::Error>`），`?` 自动 `.into()`，少写 `map_err` |
| **库 (library)** | 对外暴露**具体、详尽**的 `enum`；用 **`thiserror`** 生成样板；**不要**把宏依赖 leak 到公共 API |
| **应用 (binary)** | 汇总多库异构错误 → **`anyhow::Error`** / `Box<dyn Error>`；堆栈、上下文等 |

---

## 4. 案例与代码要点

### Newtype + 空 `Error` impl

```rust
#[derive(Debug)]
pub struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}
```

### 穷尽枚举 + 溯源

```rust
#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    General(String),
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Utf8(e) => Some(e),
            MyError::General(_) => None,
        }
    }
}
```

### `From` + `?`（示意）

```rust
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

fn read_config() -> Result<Config, MyError> {
    let s = std::fs::read_to_string("cfg.toml")?; // 自动 Io → MyError
    Ok(parse(&s)?)
}
```

---

## 5. 易错细节

| 误区 | 说明 |
|------|------|
| **`type MyError = String`** | 类型别名**不是新类型**，仍不能 `impl Error for MyError` |
| **`no_std`** | 稳定版上 `Error` 在 `std`，嵌入式可能用不了；关注 `core::error::Error` 演进 |
| **手写 `From<E> for Wrapped` 泛型** | 易与 `impl<T> From<T> for T` 冲突 → **E0119**；应用层用 **anyhow** |
| **库里的 `unwrap` / `String` 错误** | 公共 API 应给调用方可恢复、可匹配的错误类型 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 04](../ER-拓展索引.md#item-04)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 惯例 | 自定义 `E` 应 `Display + Debug + Error` |
| 孤儿 | 不能 `impl Error for String`；用 Newtype / enum |
| 库 | 具体 `enum` + `thiserror` |
| 应用 | `anyhow` / `Box<dyn Error>` |
| `?` | `From` 子错误，少 `map_err` |
| `source` | 链式暴露底层原因 |
