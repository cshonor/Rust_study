# 1.1 错误链：`Error::source()` 与 `thiserror` / `anyhow`

> 所属：**Representing Errors** · 父节：[01 枚举式错误](./01-enumeration.md) · [← 章索引](./README.md)  
> 线程 panic 统一处理 → [1.1.2.3 anyhow](../../../04-Async-Concurrency-Network/01-atomic/Chapter-01-Rust-Concurrency-Basics/1.1-threads-in-rust/1.1.2.3-panic-box-dyn-any.md) · ER 详解 → [Item 04 source()](../../01-ER/Chapter-01-Types/Item-04-idiomatic-error-types/01-core-concepts.md)

---

## 一句话

Rust **标准库**用 **`Error::source()`** 串错误链：**上层**业务错误（你定义的 `StrategyError`）的 **`source()` 指向其包装的底层官方根错误**（`reqwest::Error`、`std::io::Error` …）。**`thiserror`** 自动生成这条指针；**`anyhow::chain()`** 沿同一条单链表往下挖。

---

## 上下层别搞反：谁是根、谁是包装

| 层级 | 是什么 | 量化例子 | 谁写 |
|------|--------|----------|------|
| **底层（根错误）** | 最原始、官方/生态 **`Error`** | `reqwest::Error`（连接超时）、`std::io::Error`（文件不存在） | 标准库 / crate |
| **上层（包装错误）** | 你定义的 **业务 enum** | `StrategyError::FetchQuoteFailed` — 「行情拉取失败」 | 你的策略 crate |

**`source()` 的方向**（从调用方第一眼看到的错误往下挖）：

```text
  上层（对外）                         底层（根因）
  ───────────                         ───────────
  StrategyError::FetchQuoteFailed     reqwest::Error（连接超时）
  Display：「行情拉取失败」                  │
         │                                 │ 再无 source
         │  source() → Some(&reqwest_err)  ▼
         └──────────────────────────────  None
```

- **不是**「底层指向上层」— 永远是 **上层包装** 通过 **`source()`** 指向 **它包进去的那一份底层官方错误**。
- 顺着 **`source()`** 走：**业务语义 → 技术根因 → `None`**，一条单链表。

---

## 分层价值：同一底层技术错误，多种业务包装

自定义错误 + **`source()`** 的核心价值：**底层可以是同一种官方错误，上层按业务场景包成不同变体** — 对外语义不同，排障时仍能 **`source()`** 挖到同一类技术根因（DNS 失败、TCP 超时等）。

**量化实盘**：底层都是 **`reqwest::Error`**，上层按场景拆成三种 **`StrategyError`**：

```text
                    reqwest::Error（底层类型相同：超时 / DNS / 握手…）
                           ▲
           ┌───────────────┼───────────────┐
           │ source()      │ source()      │ source()
           │               │               │
  FetchQuoteFailed   LoginFailed   HistoryDataFailed
  「拉行情失败」      「登交易失败」   「拉历史数据失败」
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrategyError {
    #[error("行情拉取失败")]
    FetchQuoteFailed(#[from] reqwest::Error),

    #[error("交易接口登录失败")]
    LoginFailed(#[from] reqwest::Error),

    #[error("历史数据拉取失败")]
    HistoryDataFailed(#[from] reqwest::Error),
}

fn fetch_quotes() -> Result<(), StrategyError> {
    reqwest::blocking::get(QUOTE_URL)?; // 失败 → FetchQuoteFailed(reqwest_err)
    Ok(())
}

fn login_trading() -> Result<(), StrategyError> {
    reqwest::blocking::post(LOGIN_URL)?; // 失败 → LoginFailed(reqwest_err)
    Ok(())
}
```

| 上层变体（调用方 `match`） | 业务含义 | 典型动作 |
|----------------------------|----------|----------|
| **`FetchQuoteFailed`** | 拉行情这条链路挂了 | 切备用行情源、暂停策略 |
| **`LoginFailed`** | 登交易接口挂了 | 重登、换席位、告警运维 |
| **`HistoryDataFailed`** | 补历史 K 线挂了 | 降级用本地缓存、延后回测 |

- **调用方**：只 **`match` 上层变体** — **不必**区分底层是 DNS 还是 TCP 超时；看到 **`FetchQuoteFailed`** 就知道走「切备用源」。
- **排障 / 日志**：对任意变体 **`e.source()`** → 同类型的 **`reqwest::Error`**，看具体超时/证书/HTTP 状态码。
- **分层分工**：**底层**管「发生了什么技术问题」；**上层**管「这个问题在我的业务里意味着什么」。

> 运行时：每次失败各包**各自**的一份 **`reqwest::Error` 实例**；「同一个底层」指的是**同一错误类型**，不是全局共享一个对象。每个上层变体的 **`source()`** 指向**本变体里 embedded 的那一份** reqwest 错误。

---

## 心智模型：`source()` = 上层握有的「根因指针」

每个实现了 **`Error`** 的**上层**类型，通过 **`source()`** 指向**它包装的那一个底层根错误**（不是指向上层）：

```text
StrategyError::FetchQuoteFailed    ← 上层 Display：「行情拉取失败」（业务告警 / 切备用源）
        │
        │ source()  →  Some(&reqwest_error)
        ▼
reqwest::Error                     ← 底层根错误：「connection timed out」
        │
        │ source()  →  None（官方错误链在此结束或继续其自有 source）
        ▼
      （结束）
```

| 角色 | 谁看到什么 |
|------|------------|
| **调用方读 `Display`** | **上层**业务提示 — 「行情拉取失败」→ `match` 告警、切备用源 |
| **调用方调 `source()`** | **底层**技术根因 — `reqwest` 超时、`io` 文件不存在 |
| **若不实现 `source()`** | 链在**上层**被掐断 — 只剩业务人话，挖不到官方根错误 |

**关键**：`source()` 是 **`Option<&dyn Error>`** — 每个上层错误**至多指向一个**底层根因；反复调用得到**一条**溯源路径（单链表，不是多叉树）。

### 实盘：子线程拉行情 → 包成 `StrategyError` → 主线程 `match` + `source()`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrategyError {
    #[error("行情拉取失败")]
    FetchQuoteFailed(#[from] reqwest::Error),

    #[error("交易接口登录失败")]
    LoginFailed(#[from] reqwest::Error),

    #[error("历史数据拉取失败")]
    HistoryDataFailed(#[from] reqwest::Error),

    #[error("加载策略数据失败")]
    DataLoadError(#[from] std::io::Error),
}

// 子线程：reqwest 失败 → ? 包成上层 StrategyError
fn worker_fetch() -> Result<(), StrategyError> {
    let _ = reqwest::blocking::get("https://quote-api/…")?;
    Ok(())
}

// 主线程：match 上层业务变体；source() 挖底层 reqwest 细节
fn on_worker_err(e: &StrategyError) {
    match e {
        StrategyError::FetchQuoteFailed(_) => {
            alert("行情拉取失败，切备用源");
        }
        StrategyError::LoginFailed(_) => {
            alert("交易登录失败，触发重登");
        }
        StrategyError::HistoryDataFailed(_) => {
            alert("历史数据失败，降级本地缓存");
        }
        StrategyError::DataLoadError(_) => { /* … */ }
    }
    if let Some(root) = e.source() {
        eprintln!("  技术根因: {root}"); // 多为 reqwest::Error 或 io::Error
    }
}
```

1. **先**在底层产生 **`reqwest::Error`**（官方原始错误）。
2. **`?` / `map_err`** 包成 **上层 `StrategyError::FetchQuoteFailed`** — 抛给主线程的是**业务可 match** 的类型。
3. **`source()`** 从上层指回 **`reqwest::Error`** — 告警用上层，排障挖底层；**`thiserror` 的 `#[from]` 自动写好 `source`**，不必手动存字段再 `impl source`。

### 单链表，不是多叉树

口语里有时把多层 context 比作「树」，但 **`Error::source()` 的 API 是单父指针** — 一个上层错误通常只对应**一个最直接的根因**：

```text
  多叉树（❌ 不是 source() 模型）     单链表（✅ 错误链：从上往下挖根）
        A                              StrategyError（上层）
       / \                                  │ source()
      B   C                                 ▼
                                     reqwest / io（底层根）
                                              │ source()
                                              ▼
                                            None
```

| | **`source()` 链** | 多分支「树」想象 |
|---|-------------------|------------------|
| **每个节点的父** | **至多 1 个**（`Option`） | 可有多个子节点 |
| **遍历方式** | 反复 **`source()`** → 线性 **溯源路径** | 需 DFS/BFS |
| **实现约定** | **上层** `source()` 只 **`Some` 一个底层根错误** | — |

**`anyhow` 的 `chain()`** 打印像「多层栈」，底层仍是 **parent 指针串成的一条链**（外加 context 层），不是同一层多个并列根因。

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

手写时要自己 **存 `io` 字段** + **写 `match source()`** → 用 **`thiserror`** 时 **`#[source]` / `#[from]`** 自动生成（见下）；**不必每个变体都从零 `impl source`**。

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
| **上层（包装）** | 你的 **`StrategyError`** 等 — 业务语义，给 `match` / 告警 |
| **底层（根）** | **`source()` 指向的对象** — `reqwest::Error`、`io::Error` 等官方错误 |
| **链** | 从**上层**反复 **`source()`** 直到 **`None`** — 单链表，方向**从上往下**挖根 |

```text
StrategyError::FetchQuoteFailed   ← 上层 Display：「行情拉取失败」
           │ source()
           ▼
    reqwest::Error（连接超时）     ← 底层根错误
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

### `thiserror`：`#[source]`、`#[from]` 自动挂父指针

**不必**给每个自定义错误从零写 **`impl Error` + `source()`** — 在**顶层业务 enum** 上加好变体，关联逻辑交给宏：

| 属性 | 自动生成 |
|------|----------|
| **`#[source]`**（命名字段） | 该字段 → **`source()` 的 `Some(&field)`** |
| **`#[from]`**（单字段/tuple 变体） | **`From<底层>`** + **`source()` 指向该字段**（`#[from]` 隐含 source） |
| **`#[error("…")]`** | **`Display`** |

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrategyError {
    #[error("加载策略数据失败")]
    DataLoadError {
        #[source]
        io: std::io::Error,
    },

    // 上层包装 ← 底层 reqwest；? 自动 From + source 指根
    #[error("行情拉取失败")]
    FetchQuoteFailed(#[from] reqwest::Error),

    #[error("策略 {0} 参数非法")]
    BadParam(String), // 无底层 → source() = None
}

fn pull_quotes() -> Result<(), StrategyError> {
    let _body = reqwest::blocking::get("https://…")?; // ? → StrategyError::FetchQuoteFailed
    Ok(())
}
```

**`FetchQuoteFailed(#[from] reqwest::Error)` 一行等价于**：

1. 变体里**存** `reqwest::Error`；
2. **`impl From<reqwest::Error> for StrategyError`** — `?` 能向上转；
3. **`source()`** 在**上层变体**上返回 **`Some(&reqwest_error)`** — 指针方向：**包装 → 根**。

你只需维护**最顶层** `StrategyError` 的变体与文案；**父指针 + From** 由宏生成，不用手写一堆 `match source()`。

| 手写 | `thiserror` |
|------|-------------|
| 变体里存底层错误 | 字段 / `#[from]` tuple |
| `impl Error { fn source() … }` | **`#[source]`** 或 **`#[from]`** |
| `impl From<底层>` | **`#[from]`** |
| `impl Display` | **`#[error("…")]`** |

- 库 crate 公开 API 仍用 **enum + `thiserror`**（调用方可 `match` 分支）。ER 对照 → [Item 04 §thiserror](../../01-ER/Chapter-01-Types/Item-04-idiomatic-error-types/04-examples.md#thiserror自动实现-error库推荐)。

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
主线程 / anyhow context（更外层，可选）
    → source()
StrategyError::FetchQuoteFailed   （上层，thiserror）
    → source()
reqwest::Error / io::Error        （底层，官方根错误）
    → source()
None
    ‖ 可恢复路径用 Result + ?
join panic → downcast → anyhow     （panic 路径，见 1.1.2.3）
```

---

## 选型速记

| 场景 | 用 |
|------|-----|
| 库 API、调用方要 `match` 变体 | **`enum` + `thiserror`** + **`#[from]` / `#[source]`** |
| bin / 策略主流程、多 crate 混用 `?` | **`anyhow::Result`** + `.context()` |
| 只要.walk 底层原因 | **`Error::source()`** 循环或 **`anyhow::chain()`** |
| 子线程已 panic | **`downcast_ref`** → 再 **`anyhow!`** 并进链（不是 `source()` 自动连 panic） |

---

## 相关

- [01 枚举式错误](./01-enumeration.md) · [02 不透明错误](./02-opaque-errors.md)
- [04 传播错误](./04-propagating-errors.md)
- [ER Item 04](../../01-ER/Chapter-01-Types/Item-04-idiomatic-error-types/README.md)

§4 索引：[README.md](./README.md)
