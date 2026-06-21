# 1.1 错误链：`Error::source()` 与 `thiserror` / `anyhow`

> 所属：**Representing Errors** · 父节：[01 枚举式错误](./01-enumeration.md) · [← 章索引](./README.md)  
> 线程 panic 统一处理 → [1.1.2.3 anyhow](../../../04-Async-Concurrency-Network/01-atomic/Chapter-01-Rust-Concurrency-Basics/1.1-threads-in-rust/1.1.2.3-panic-box-dyn-any.md) · ER 详解 → [Item 04 source()](../../01-ER/Chapter-01-Types/Item-04-idiomatic-error-types/01-core-concepts.md)

---

## 一句话

Rust **标准库**用 **`std::error::Error::source()`** 表达「上层错误 ← 底层原因」；心智上就是给错误加了一个 **「父错误指针」** — **`thiserror`** 自动生成这条指针，**`anyhow::Error`** 用 **`chain()`** 遍历整条链。

---

## 心智模型：`source()` = 「父错误指针」

每个实现了 **`Error`** 的类型，都可以选择通过 **`source()`** 指向**自己的源头**（上一层 / 更底层的原因）：

```text
StrategyError::DataLoadError          ← 业务层 Display：「加载策略数据失败」
        │
        │ source()  ≈  「父错误指针」→ Some(&io_error)
        ▼
std::io::Error                        ← 技术根因：「文件不存在」
        │
        │ source()  → None（链顶之下无父）
        ▼
      （结束）
```

| 角色 | 谁看到什么 |
|------|------------|
| **调用方读 `Display`** | 你定义的**业务提示** — 「加载策略数据失败」 |
| **调用方调 `source()`** | **技术根因** — 最初的 `io::Error`（文件不存在等） |
| **若不实现 `source()`** | 链在业务层**被掐断** — 只剩一句人话，挖不到根因 |

**关键**：`source()` 返回的是 **`Option<&dyn Error>`** — 一条**单向**父指针，不是双向链表；沿着它走不会丢上下文。

### 量化示例：CSV 行情加载（手写 `source`）

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum StrategyError {
    DataLoadError { io: std::io::Error },
    BadParam(String),
}

impl fmt::Display for StrategyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataLoadError { .. } => write!(f, "加载策略数据失败"),
            Self::BadParam(p) => write!(f, "策略参数非法: {p}"),
        }
    }
}

impl Error for StrategyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // 给 DataLoadError 挂上指向原始 IO 错误的「父指针」
            Self::DataLoadError { io } => Some(io),
            Self::BadParam(_) => None,
        }
    }
}

fn load_csv(path: &str) -> Result<Vec<u8>, StrategyError> {
    std::fs::read(path).map_err(|io| StrategyError::DataLoadError { io })
}
```

别人拿到 **`StrategyError::DataLoadError`** 时：表面是「加载策略数据失败」，**`source()`** 一路挖到 **`NotFound`** 等 OS 级原因 — **链不断**。

手写时要自己 **存 `io` 字段** + **写 `match source()`** → **`thiserror`** 把「存字段 + 挂父指针」合成 **`#[source]`** 一行（见下）。

---

## 原生机制：`source()` 拼错误链

```rust
pub trait Error: Display + Debug {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None // 默认无下层
    }
}
```

| 概念 | 说明 |
|------|------|
| **上层错误** | 对外语义 — 「行情获取失败」「策略初始化失败」 |
| **底层错误** | `source()` 指向的 **`dyn Error`** — 如 `io::Error`、网络超时 |
| **链** | 每层再调 **`source()`**，直到 `None` — **单向链表**，不是循环 |

```text
调用方看到的 Display
    「行情获取失败」
           │ source()
           ▼
    reqwest / io 超时
           │ source()
           ▼
         None
```

### 量化示例（可恢复路径 · `Result`）

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MarketFetchError {
    detail: String,
    source: std::io::Error,
}

impl fmt::Display for MarketFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "行情获取失败: {}", self.detail)
    }
}

impl Error for MarketFetchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source) // 把 CTP/TCP 底层错误链出去
    }
}

fn dig_root(mut e: &dyn Error) {
    while let Some(cause) = e.source() {
        eprintln!("  caused by: {cause}");
        e = cause;
    }
}
```

手写 `Display` / `Error` / `source()` 重复多 → 用 **`thiserror`**（见下）。

---

## 生态替代：`failure` / `error-chain` → 标准库 + `thiserror` / `anyhow`

| 年代 / 方案 | 现状 |
|-------------|------|
| **`failure`**、**`error-chain`** | 已基本**弃用**；错误链能力已进 **std `Error::source()`** |
| **库（对外 enum）** | **`thiserror`** — `#[derive(Error)]` + `#[source]` 映射变体字段 |
| **应用 / bin（不透明汇总）** | **`anyhow`** — `anyhow::Error` 内部包装多源错误，**`chain()`** 遍历 |

### `thiserror`：自动写好「父错误指针」

不用手动 **`impl Error`** 里的 **`source()` match**，也不用重复写 Display 样板 — 在字段上加 **`#[source]`**，宏生成：**存底层错误 + `source()` 返回 `Some(&that_field)`**。

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrategyError {
    #[error("加载策略数据失败")]
    DataLoadError {
        #[source]
        io: std::io::Error, // 读行情 CSV 失败 — 父指针自动指向 io
    },
    #[error("行情获取失败")]
    MarketFetch {
        #[source]
        source: std::io::Error,
    },
    #[error("策略 {0} 参数非法")]
    BadParam(String), // 无 #[source] → source() 为 None
}
```

| 手写 | `thiserror` |
|------|-------------|
| 变体里存 `io: std::io::Error` | 同 — 字段保留 |
| `impl Error { fn source() { Some(&io) } }` | **`#[source]`** 自动生成 |
| `impl Display` | **`#[error("…")]`** 自动生成 |

- 库 crate 公开 API 仍用 **enum + `thiserror`**（调用方可 `match` 分支）。

### `anyhow`：一条链打印 / 遍历

```rust
use anyhow::{Context, Result};

fn fetch_bars() -> Result<Vec<u8>> {
    tcp_connect().context("连接 CTP 行情网关")?;
    Ok(vec![])
}

fn tcp_connect() -> Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout"))?;
    Ok(())
}

fn main() {
    if let Err(e) = fetch_bars().context("run mean_revert_v2") {
        // Debug 格式常带多层 context
        eprintln!("{e:?}");
        for cause in e.chain() {
            eprintln!("  caused by: {cause}");
        }
    }
}
```

Demo：[Item 04 demo](../../01-ER/Chapter-01-Types/Item-04-idiomatic-error-types/demo/src/main.rs)（`e.chain()` 示例）。

---

## 与线程 `join` / panic 路径的对照

| 路径 | 容器 | 如何「挖原因」 |
|------|------|----------------|
| **`Result` + `?`（业务可恢复）** | `anyhow::Error` / `thiserror` enum | **`source()`** 或 **`anyhow::Error::chain()`** |
| **子线程 panic → `join` Err** | `Box<dyn Any + Send + 'static>` | **`downcast_ref`**（类型擦除，非 `source` 链）→ [1.1.2.3](../../../04-Async-Concurrency-Network/01-atomic/Chapter-01-Rust-Concurrency-Basics/1.1-threads-in-rust/1.1.2.3-panic-box-dyn-any.md) · [1.1.2.4 downcast](../../../04-Async-Concurrency-Network/01-atomic/Chapter-01-Rust-Concurrency-Basics/1.1-threads-in-rust/1.1.2.4-panic-capture-downcast.md) |

**统一处理策略**（量化主流程常见写法）：

1. **可恢复** — 数据源 / 计算 / 配置 → **`anyhow::Result` + `?`**，日志里 **`{err:?}`** 或 **`chain()`** 自动带栈式 context。
2. **不可恢复（worker panic）** — `join_or_anyhow` 把 panic 载荷 **转成 `anyhow::Error`** 再 `?`，与①同型 → [1.1.2.3 §anyhow](../../../04-Async-Concurrency-Network/01-atomic/Chapter-01-Rust-Concurrency-Basics/1.1-threads-in-rust/1.1.2.3-panic-box-dyn-any.md#与-anyhow把-join-的-err-并进-result---主流程)。
3. **库边界** — 对外仍 **`thiserror` enum**；应用层 `?` 进 **`anyhow`** 时再 losing 具体变体，换组合性。

```text
CTP io::Error
    → source() / context
StrategyError::MarketFetch  （库，thiserror）
    → ? 进 anyhow
anyhow::Error  「run strategy」→ chain() 一次看完
    ‖ 同型
join panic → map_err → anyhow::Error
```

---

## 选型速记

| 场景 | 用 |
|------|-----|
| 库 API、调用方要 `match` 变体 | **`enum` + `thiserror`** + `#[source]` |
| bin / 策略主流程、多 crate 混用 `?` | **`anyhow::Result`** + `.context()` |
| 只要.walk 底层原因 | **`Error::source()`** 循环或 **`anyhow::chain()`** |
| 子线程已 panic | **`downcast_ref`** → 再 **`anyhow!`** 并进链（不是 `source()` 自动连 panic） |

---

## 相关

- [01 枚举式错误](./01-enumeration.md) · [02 不透明错误](./02-opaque-errors.md)
- [04 传播错误](./04-propagating-errors.md)
- [ER Item 04](../../01-ER/Chapter-01-Types/Item-04-idiomatic-error-types/README.md)

§4 索引：[README.md](./README.md)
